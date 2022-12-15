// https://www.sheshbabu.com/posts/rust-module-system/

pub mod pattern;
pub mod dag_creator;
pub mod dag;

use debug_print::{debug_println};
use std::fs;
use pattern::*;
use dag_creator::*;
use dag::*;

pub fn getPatterns(path:String) -> Vec<Pattern>{
    let mut patterns: Vec<Pattern> = Vec::new();        
    let lines: Vec<String> = fs::read_to_string(path)
        .expect("File not found")
        .split("\n")
        .map(|i| i.to_owned())
        .collect();

    for (i, line) in lines.iter().enumerate() {
        if line.trim().is_empty(){
            continue;
        }
        patterns.push(Pattern::new(i as u32 + 1, line.to_owned()));
    }

    return patterns;
}

pub fn main() {
    // let path = "tests/test_data/real1.txt".to_owned(); // 658 fonts, elapsed time: 8m
    // let path = "tests/test_data/4k-big-patterns.txt".to_owned(); // 658 fonts, elapsed time: 8m
    // let path = "tests/test_data/9k-small-patterns.txt".to_owned();
    let path = "tests/test_data/real1.txt".to_owned(); // [2, 1, 4, 3] [1, 2, 4, 3]
    
    let patterns = getPatterns(path);

    // dbg!(patterns.get(1).unwrap().selfRelationTo(patterns.get(4).unwrap()));
    // dbg!(patterns.get(2).unwrap().selfRelationTo(patterns.get(0).unwrap()));
    // dbg!(patterns.get(2).unwrap().selfRelationTo(patterns.get(3).unwrap()));
    // dbg!(patterns.get(3).unwrap().selfRelationTo(patterns.get(0).unwrap()));
    // println!();
    // dbg!(patterns.get(1).unwrap().selfRelationTo(patterns.get(2).unwrap()));
    // dbg!(patterns.get(1).unwrap().selfRelationTo(patterns.get(3).unwrap()));
    // dbg!(patterns.get(1).unwrap().selfRelationTo(patterns.get(0).unwrap()));

    let mut dag_creator = DagCreator::new(patterns);
    dag_creator.create();
}