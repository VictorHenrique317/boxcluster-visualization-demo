// https://www.sheshbabu.com/posts/rust-module-system/

pub mod pattern;
pub mod dag_creator;

use debug_print::{debug_println};
use std::fs;
use pattern::*;
use dag_creator::*;

pub fn getPatterns(path:String) -> Vec<Pattern>{
    let mut patterns: Vec<Pattern> = Vec::new();        
    let lines: Vec<String> = fs::read_to_string(path)
        .expect("File not found")
        .split("\n")
        .map(|i| i.to_owned())
        .collect();

    for (i, line) in lines.iter().enumerate() {
        patterns.push(Pattern::new(i as u32 + 1, line.to_owned()));
    }

    return patterns;
}

pub fn main() {
    let path = "tests/test_data/complex-msub.txt".to_owned();
    let patterns = getPatterns(path);

    let mut dag_creator = DagCreator::new();
    dag_creator.calculate(patterns, None);
}