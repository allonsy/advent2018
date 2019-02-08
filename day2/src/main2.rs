use std::collections::HashSet;
use util;

fn main() {
    let words = get_words();
    let mut sim_map = HashSet::new();
    for word in &words {
        match process_word(word, &mut sim_map) {
            Some(match_str) => {
                println!("matching word is: {}", match_str);
                return;
            }
            None => {}
        };
    }
    println!("No matches found!");
}

fn process_word(word: &String, sim_map: &mut HashSet<String>) -> Option<String> {
    let wordlen = word.len();
    let mut word_sim_map = HashSet::new();
    for i in 0..wordlen {
        let mut new_str = word.clone();
        new_str.remove(i);
        if sim_map.contains(&new_str) {
            return Some(new_str);
        } else {
            word_sim_map.insert(new_str);
        }
    }

    for val in word_sim_map {
        sim_map.insert(val);
    }
    return None;
}

fn get_words() -> Vec<String> {
    return util::read_file_lines("input.txt");
}
