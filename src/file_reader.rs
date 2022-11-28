#![allow(non_snake_case)]
// #[path = "pattern.rs"] mod pattern;

use crate::Pattern;
use std::fs;

pub fn readPatternFile(path: String) -> Vec<Pattern>{
    let mut patterns: Vec<Pattern> = Vec::new();

    let lines: Vec<String> = fs::read_to_string(path)
        .expect("File not found")
        .split("\n")
        .map(|i| i.to_owned())
        .collect();

    for line in lines.iter() {
        patterns.push(Pattern::new(line.to_owned()));
    }

    return patterns;
}
