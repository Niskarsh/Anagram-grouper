// Given a list of words, group the anagrams
use anagrams::anagramGrouper;
fn main() {
    let words = vec!["the".to_string(), "teh".to_string(), "hte".to_string(), "apple".to_string(), "appel".to_string()];
    let output = anagramGrouper(&words);
    println!("{:?}", output);
}
