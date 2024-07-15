use clap::Parser;
use core::{
    common::{all_fingerprint, fingerprint, Fingerprint},
    token::Token,
};
use indicatif::ProgressIterator;
use log::*;
use regex::Regex;
use std::{collections::HashMap, fs::read_dir, path::PathBuf};
use walkdir::WalkDir;

#[derive(Parser)]
struct Args {
    /// Path to source directory
    #[arg(short, long)]
    source_directory: PathBuf,

    #[arg(short, long)]
    reference_directory: Option<PathBuf>,

    /// Path to template directory
    #[arg(short = 'T', long)]
    template_directory: PathBuf,

    /// Regex patterns for files to include
    #[arg(short, long)]
    include: Vec<Regex>,

    #[arg(short = 'n', long, default_value_t = 40)]
    number_of_report: usize,

    #[arg(short = 'c', long, default_value_t = 10)]
    common_cutoff: usize,

    #[arg(short='N', long, default_value_t = 40)]
    winnow_noise: usize,

    #[arg(short='G', long, default_value_t = 80)]
    winnow_guarantee: usize,
}

fn main() -> anyhow::Result<()> {
    let opts = Args::parse();
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
            match core::lang::tokenize(path) {
                Ok(tokens) => {
                    template_tokens.insert(relative_path.to_path_buf(), tokens);
                }
                Err(err) => {
                    warn!("Tokenize {} failed with {}", path.display(), err);
                }
            }
        }
    }

    let in_reference_dir = |path: &PathBuf| {
        opts.reference_directory
            .as_ref()
            .map_or(false, |dir| path.starts_with(dir))
    };

    // walk source directory
    info!("Processing source directory");
    let submissions = read_dir(&opts.source_directory).unwrap();
    let references = opts.reference_directory.as_ref().map_or(Vec::new(), |dir| {
        read_dir(&dir).unwrap().collect::<Vec<_>>()
    });
    // map: file => submission => tokens
    let mut all_tokens: HashMap<PathBuf, HashMap<PathBuf, Vec<Token>>> = HashMap::new();
    for submission in submissions
        .chain(references)
        .collect::<Vec<_>>()
        .into_iter()
        .progress()
    {
        let submission = submission?;
        if !submission.file_type()?.is_dir() {
            continue;
        }
        let submission_directory = submission.path();
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
                match core::lang::tokenize(path) {
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

    for submission in all_tokens.keys() {
        info!("Processing file {}", submission.display());
        let keys: Vec<&PathBuf> = all_tokens[submission].keys().collect();

        if !template_tokens.contains_key(submission) {
            continue;
        }

        // https://theory.stanford.edu/~aiken/publications/papers/sigmod03.pdf
        let template_token = &template_tokens[submission];
        let template_fingerprint = all_fingerprint(template_token.iter().map(|t| t.kind), opts.winnow_noise);
        let mut local_tokens = vec![];
        let mut local_fingerprints = vec![];
        let mut index: HashMap<u64, Vec<(Fingerprint, usize)>> = HashMap::new();
        for (i, key) in keys.iter().enumerate() {
            let token = all_tokens[submission][*key].clone();
            let fingerprint = fingerprint(token.iter().map(|t| t.kind), opts.winnow_noise, opts.winnow_guarantee);
            debug!(
                "{}: {} tokens, {} fingerprints",
                keys[i].display(),
                token.len(),
                fingerprint.len()
            );
            // insert to index: fingerprint => f
            for f in &fingerprint {
                index.entry(f.hash).or_default().push((*f, i));
            }
            local_fingerprints.push(fingerprint);
            local_tokens.push(token);
        }

        // exclude fingerprints in template
        for f in &template_fingerprint {
            index.remove(&f.hash);
        }

        // create two dimensional matrix
        let mut m = vec![0; keys.len() * keys.len()];
        for hash in index.keys() {
            let v = &index[hash];
            if v.len() > opts.common_cutoff {
                // too common, skip
                continue;
            }

            if v.len() > 5 {
                debug!("Found {} entries:", v.len());
                for (f, i) in v {
                    debug!(
                        "{} offset {} L{} C{}",
                        keys[*i].display(),
                        f.offset,
                        local_tokens[*i][f.offset].line,
                        local_tokens[*i][f.offset].column,
                    );
                }
            }
            // add to matrix
            for i in 0..v.len() {
                for j in (i + 1)..v.len() {
                    if v[i].1 == v[j].1 {
                        continue;
                    }
                    m[v[i].1 * keys.len() + v[j].1] += 1;
                    m[v[j].1 * keys.len() + v[i].1] += 1;
                }
            }
        }

        let mut sorted_m: Vec<_> = m.iter().enumerate().collect();
        sorted_m.sort_by_key(|(_, val)| **val);
        for (left, right, matches) in sorted_m
            .iter()
            .rev()
            .map(|(i, matches)| (i % keys.len(), i / keys.len(), **matches))
            .filter(|(left, right, _)| {
                let left = *left;
                let right = *right;
                left < right && !(in_reference_dir(keys[left]) && in_reference_dir(keys[right]))
            })
            .take(opts.number_of_report)
        {
            // show info
            info!(
                "Possible plagarism: {} and {}: {} matches",
                keys[left].display(),
                keys[right].display(),
                matches,
            );
        }
    }
    Ok(())
}
