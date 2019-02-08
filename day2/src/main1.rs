use std::collections::HashMap;
use util;

fn main() {
    let words = get_words();

    let mut num_doubles = 0;
    let mut num_triples = 0;
    for word in &words {
        let (has_double, has_triple) = process_word(word);
        if has_double {
            num_doubles += 1;
        }
        if has_triple {
            num_triples += 1;
        }
    }

    println!("checksum is: {}", num_doubles * num_triples);
}

fn process_word(word: &String) -> (bool, bool) {
    let mut freq_map: HashMap<u8, u32> = HashMap::new();

    for byte in word.as_bytes() {
        let count = freq_map.entry(*byte).or_insert(0);
        *count += 1;
    }

    let mut has_double = false;
    let mut has_triple = false;

    for (ch, freq) in &freq_map {
        if *freq == 2 {
            has_double = true;
        } else if *freq == 3 {
            has_triple = true;
        }
    }

    (has_double, has_triple)
}

fn get_words() -> Vec<String> {
    return util::read_file_lines("input.txt");
}
