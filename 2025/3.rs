use std::{
    env::args,
    fs::File,
    io::{BufRead, BufReader},
};

fn calculate_joltage(nums: &[usize], batteries: usize) -> usize {
    let mut highest_left_index = 0;
    let mut highest_nums = Vec::new();
    while highest_nums.len() != batteries {
        for i in highest_left_index..nums.len() {
            let left = nums.len() - i;
            if left < batteries - highest_nums.len() {
                break;
            }

            if nums[highest_left_index] < nums[i] {
                highest_left_index = i;
            }
        }

        dbg!(highest_left_index);
        highest_nums.push(nums[highest_left_index]);
        highest_left_index += 1;
    }

    highest_nums.iter().enumerate().fold(0, |acc, (i, v)| {
        acc + *v * 10_usize.pow((highest_nums.len() - i - 1) as u32)
    })
}

fn main() {
    let filename = args().collect::<Vec<_>>();
    let filename = filename.get(1).expect("filename provided");
    let file = File::open(filename).expect("file opens");
    let mut file = BufReader::new(file);

    let mut acc: usize = 0;

    let mut buf = String::new();
    loop {
        buf.clear();
        if file.read_line(&mut buf).unwrap() == 0 {
            break;
        }

        let string = buf.trim_ascii();
        if string.len() == 0 {
            continue;
        }

        let nums = string
            .chars()
            .map(|v| v.to_digit(10).unwrap() as usize)
            .collect::<Vec<_>>();

        acc += calculate_joltage(&nums, 12);
    }

    println!("{}", acc);
}
