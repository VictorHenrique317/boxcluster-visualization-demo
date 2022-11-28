mod pattern;
use pattern::Pattern;

fn main() {
    println!("Hello, world!");

    let pattern_str: String = "1,2,3 3,2,1".to_owned();
    for i in pattern_str.split(" "){
        println!("{}", i);
    }
}
