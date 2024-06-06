use clap::Parser;
use core::{common::gen_svg_with_index, lang::tokenize, matching::compute_matches_from_token};
use random_color::{Luminosity, RandomColor};
use rkr_gst::Match;
use std::{
    fs::File,
    io::{Read, Write},
    path::{Path, PathBuf},
};

#[derive(Parser)]
struct Args {
    /// Path to left source
    #[arg(short, long)]
    left: PathBuf,

    /// Path to right source
    #[arg(short, long)]
    right: PathBuf,

    /// Path to template source
    #[arg(short, long)]
    template: Option<PathBuf>,
}

fn read_file_lines(s: &Path) -> anyhow::Result<Vec<String>> {
    let mut file = File::open(s)?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;
    Ok(s.lines().map(String::from).collect::<Vec<String>>())
}

fn main() -> anyhow::Result<()> {
    let opts = Args::parse();
    env_logger::init();

    let token_left = core::lang::tokenize(&opts.left).unwrap();
    let token_kind_left: Vec<u8> = token_left.iter().map(|t| t.kind).collect();
    let lines_left = read_file_lines(&opts.left)?;

    let token_right: Vec<core::token::Token> = core::lang::tokenize(&opts.right).unwrap();
    let token_kind_right: Vec<u8> = token_right.iter().map(|t| t.kind).collect();
    let lines_right = read_file_lines(&opts.right)?;

    let template_kind: Option<Vec<u8>> = opts.template.as_ref().map(|t| {
        let token_template = tokenize(t).unwrap();
        token_template.iter().map(|t| t.kind).collect()
    });

    let matches = compute_matches_from_token(
        &token_left,
        &token_kind_left,
        &lines_left.iter().map(|l| l.as_str()).collect::<Vec<&str>>(),
        &token_right,
        &token_kind_right,
        &lines_right
            .iter()
            .map(|l| l.as_str())
            .collect::<Vec<&str>>(),
        template_kind.as_deref(),
    );

    for is_left in [true, false] {
        let side = if is_left { "left" } else { "right" };
        let mut file = File::create(format!("match-{}.html", side))?;
        writeln!(
            file,
            "<html><head><meta charset=\"UTF-8\"></head><body><pre>"
        )?;

        let mut last_line = 0;

        let mut matches: Vec<(usize, &Match)> = matches.iter().enumerate().collect();

        // sort by line_from
        matches.sort_by_key(|m| {
            if is_left {
                token_left[m.1.pattern_index].line as usize - 1
            } else {
                token_right[m.1.text_index].line as usize - 1
            }
        });
        let token = if is_left { &token_left } else { &token_right };
        let lines = if is_left { &lines_left } else { &lines_right };
        for (idx, m) in matches.iter() {
            let index = if is_left {
                m.pattern_index
            } else {
                m.text_index
            };
            let line_from = token[index].line as usize - 1;
            let line_to = token[index + m.length - 1].line as usize - 1;

            println!("Match #{}:", idx + 1);
            println!("L{}-L{}:", line_from, line_to);
            println!("{}", lines[line_from..=line_to].join("\n"));

            assert!(last_line <= line_from);
            assert!(line_from <= line_to);
            if last_line < line_from {
                writeln!(
                    file,
                    "{}",
                    html_escape::encode_text(&lines[last_line..=(line_from - 1)].join("\n"))
                )?;
            }
            last_line = line_to + 1;

            let color = RandomColor::new()
                .luminosity(Luminosity::Bright)
                .seed((idx + 1) as u64)
                .to_hex();
            write!(file, "<font color=\"{}\">", color)?;
            writeln!(file, "{}", gen_svg_with_index(color.as_str(), 0, Some(idx + 1)))?;
            writeln!(
                file,
                "{}",
                html_escape::encode_text(&lines[line_from..=line_to].join("\n"))
            )?;
            write!(file, "</font>")?;
        }

        if last_line < lines.len() {
            writeln!(
                file,
                "{}",
                html_escape::encode_text(&lines[last_line..].join("\n"))
            )?;
        }

        writeln!(file, "</pre></body></html>")?;
    }

    Ok(())
}
