mod pattern;
mod dag_creator;

use std::fs;
use pattern::Pattern;
use pattern::Relation;
use dag_creator::DagCreator;

fn main() {
    let mut patterns: Vec<Pattern> = Vec::new();        
    let lines: Vec<String> = fs::read_to_string("test.txt".to_owned())
        .expect("File not found")
        .split("\n")
        .map(|i| i.to_owned())
        .collect();

    for (i, line) in lines.iter().enumerate() {
        patterns.push(Pattern::new(i as u32, line.to_owned()));
    }

    // dbg!(patterns[0].selfRelationTo(&patterns[1]));
    let mut dag_creator = DagCreator::new();
    dag_creator.calculate(patterns);
}