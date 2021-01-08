use jieplag::{common::LineMatch, token::Token};
use log::*;
use regex::Regex;
use std::{
    collections::HashMap,
    fs::{read_dir, File},
    io::{Read, Write},
    path::{Path, PathBuf},
};
use structopt::StructOpt;
use walkdir::WalkDir;

#[derive(StructOpt)]
struct Args {
    /// Path to source directory
    #[structopt(short, long)]
    source_directory: PathBuf,

    /// Path to template directory
    #[structopt(short = "T", long)]
    template_directory: PathBuf,

    /// Path to result directory
    #[structopt(short, long)]
    result_directory: PathBuf,

    /// Regex patterns for files to include
    #[structopt(short, long)]
    include: Vec<Regex>,

    /// Threshold of similarity
    #[structopt(short, long, default_value = "0.6")]
    threshold: f32,
}

fn read_file_lines(s: &Path) -> anyhow::Result<Vec<String>> {
    let mut file = File::open(s)?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;
    Ok(s.lines().map(|l| String::from(l)).collect::<Vec<String>>())
}

fn main() -> anyhow::Result<()> {
    let opts = Args::from_args();
    env_logger::init();

    // walk template directory
    info!("Processing template directory");
    let mut template_tokens = HashMap::new();
    for entry in WalkDir::new(&opts.template_directory) {
        let entry = entry?;
        let path = entry.path();
        let relative_path = path.strip_prefix(&opts.template_directory)?;
        let mut include = false;
        for pattern in &opts.include {
            if pattern.is_match(&relative_path.display().to_string()) {
                include = true;
                break;
            }
        }
        if include {
            match jieplag::lang::tokenize(&path) {
                Ok(tokens) => {
                    template_tokens.insert(relative_path.to_path_buf(), tokens);
                }
                Err(err) => {
                    warn!("Tokenize {} failed with {}", path.display(), err);
                }
            }
        }
    }

    // walk source directory
    info!("Processing source directory");
    let submissions = read_dir(&opts.source_directory).unwrap();
    // map: file => submission => tokens
    let mut all_tokens: HashMap<PathBuf, HashMap<PathBuf, Vec<Token>>> = HashMap::new();
    for submission in submissions {
        let submission = submission?;
        if !submission.file_type()?.is_dir() {
            continue;
        }
        let submission_directory = opts.source_directory.join(submission.path());
        for entry in WalkDir::new(&submission_directory) {
            let entry = entry?;
            let path = entry.path();
            let relative_path = path.strip_prefix(&submission_directory)?;
            let mut include = false;
            for pattern in &opts.include {
                if pattern.is_match(&relative_path.display().to_string()) {
                    include = true;
                    break;
                }
            }
            if include {
                match jieplag::lang::tokenize(&path) {
                    Ok(tokens) => {
                        all_tokens
                            .entry(relative_path.to_path_buf())
                            .or_default()
                            .insert(submission.path(), tokens);
                    }
                    Err(err) => {
                        warn!("Tokenize {} failed with {}", path.display(), err);
                    }
                }
            }
        }
    }

    info!("Tokenized {} files in source directory", all_tokens.len());
    let mut num_match = 0;
    for submission in all_tokens.keys() {
        info!("Processing file {}", submission.display());
        let keys: Vec<&PathBuf> = all_tokens[submission].keys().collect();

        if !template_tokens.contains_key(submission) {
            continue;
        }

        let template_token = &template_tokens[submission];
        let template_token_kind: Vec<u8> = template_token.iter().map(|t| t.kind).collect();
        let mut local_tokens = vec![];
        for i in 0..keys.len() {
            let mut token = all_tokens[submission][keys[i]].clone();
            let token_kind: Vec<u8> = token.iter().map(|t| t.kind).collect();
            let mut matches = rkr_gst::run(&token_kind, &template_token_kind, 40, 20);
            matches.sort_by_key(|m| m.pattern_index);
            for m in matches.iter().rev() {
                token.drain(m.pattern_index..m.pattern_index + m.length);
            }
            local_tokens.push(token);
        }

        for left in 0..keys.len() {
            for right in (left + 1)..keys.len() {
                let token_left = &local_tokens[left];
                let token_left_kind: Vec<u8> = token_left.iter().map(|t| t.kind).collect();
                let token_right = &local_tokens[right];
                let token_right_kind: Vec<u8> = token_right.iter().map(|t| t.kind).collect();
                // too similar to template
                if token_left.len() < 1000 || token_right.len() < 1000 {
                    continue;
                }

                let mut matches = rkr_gst::run(&token_left_kind, &token_right_kind, 40, 20);

                let match_count: usize = matches.iter().map(|m| m.length).sum();

                let ratio_left = match_count as f32 / token_left.len() as f32;
                let ratio_right = match_count as f32 / token_right.len() as f32;
                if ratio_left > opts.threshold && ratio_right > opts.threshold {
                    // convert token matches to line matches
                    let mut line_matches = vec![];
                    for m in &matches {
                        line_matches.push(LineMatch {
                            left_from: token_left[m.pattern_index].line,
                            left_to: token_left[m.pattern_index + m.length - 1].line,
                            right_from: token_right[m.text_index].line,
                            right_to: token_right[m.text_index + m.length - 1].line,
                        });
                    }

                    // merge consecutive matches in line
                    let mut i = 0;
                    while i + 1 < line_matches.len() {
                        if line_matches[i].left_to == line_matches[i + 1].left_from
                            && line_matches[i].right_to == line_matches[i + 1].right_from
                        {
                            line_matches[i].left_to = line_matches[i + 1].left_to;
                            line_matches[i].right_to = line_matches[i + 1].right_to;
                            line_matches.drain(i + 1..i + 2);
                        } else {
                            i = i + 1;
                        }
                    }
                    let left_lines: Vec<String> = read_file_lines(&keys[left].join(&submission))?;
                    let right_lines: Vec<String> = read_file_lines(&keys[right].join(&submission))?;

                    // show info
                    info!(
                        "Possible plagarism: {} and {}: left {} {} right {} {}",
                        keys[left].display(),
                        keys[right].display(),
                        ratio_left,
                        token_left.len(),
                        ratio_right,
                        token_right.len(),
                    );

                    let match_file_name = opts.result_directory.join(format!("match{}", num_match));
                    let mut match_file = File::create(match_file_name)?;
                    writeln!(
                        &mut match_file,
                        "Between {} and {}: {}",
                        keys[left].display(),
                        keys[right].display(),
                        submission.display(),
                    )?;
                    matches.sort_by_key(|m| m.pattern_index);
                    for m in &line_matches {
                        writeln!(
                            &mut match_file,
                            "Left L{}-L{} match Right L{}-L{}:\n{}\n-----------------------------\n{}",
                            m.left_from,
                            m.left_to,
                            m.right_from,
                            m.right_to,
                            left_lines[m.left_from as usize..m.left_to as usize].join("\n"),
                            right_lines[m.right_from as usize..m.right_to as usize].join("\n"),
                        )?;
                    }

                    num_match += 1;
                }
            }
        }
    }
    Ok(())
}