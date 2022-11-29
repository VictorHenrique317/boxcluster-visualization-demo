mod pattern;
mod file_reader;

use pattern::Pattern;
use pattern::Relation;

fn main() {
    let patterns: Vec<Pattern> = file_reader::readPatternFile("test.txt".to_owned());
    // println!("{}", patterns[0].selfRelationTo(&patterns[1]) == Relation::NotRelatable);
    println!("{:?}", patterns[0].selfRelationTo(&patterns[1]));

}
