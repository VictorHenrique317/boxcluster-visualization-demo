mod pattern;
mod file_reader;

use pattern::Pattern;

fn main() {
    let patterns: Vec<Pattern> = file_reader::readPatternFile("test.txt".to_owned());
    patterns[0].getCells();
}
