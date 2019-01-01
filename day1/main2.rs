mod util;
use std::collections::HashSet;

fn main() {
    let mut seen_before = HashSet::new();
    let instructions = read_instructions();
    let num_instructions = instructions.len();

    let mut start_num = 0;

    let mut index = 0;
    loop {
        let n = instructions[index];
        if !seen_before.insert(start_num) {
            println!("repeated number is: {}", start_num);
            return;
        }
        start_num += n;
        index += 1;
        index = index % num_instructions;
    }
}

fn read_instructions() -> Vec<i64> {
    let lines = util::read_file_lines("input.txt");
    let mut nums = Vec::new();

    for line in lines {
        let n = line.parse::<i64>().unwrap();
        nums.push(n);
    }

    return nums;
}
