use bee::get_words;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let letters = &args[1];
    let all_words = include_str!("./word_list.txt").lines().collect();

    for (score, words) in get_words(all_words, letters).iter().rev() {
        println!("[{}]: {}", score, words.join(", "));
    }
}
