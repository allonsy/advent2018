mod util;

fn main() {
    let instructions = read_instructions();
    let mut start_num = 0;

    for n in instructions {
        start_num += n;
    }

    println!("number is {}", start_num);
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
