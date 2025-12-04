use std::{
    env::args,
    fs::File,
    io::{BufRead, BufReader},
};

fn calculate_joltage(nums: &[usize], batteries: usize) -> usize {}

// part 1
// fn calculate_num_vec(nums: &[usize]) -> usize {
//     let mut highest_index = find_highest_index(0, nums.len(), nums);
//     if highest_index == -1 {
//         return 0;
//     }
//
//     let mut second_highest_index = find_highest_index(highest_index as usize + 1, nums.len(), nums);
//     // dbg!(highest_index, second_highest_index);
//     // the last element is currently the highest
//     if second_highest_index == -1 {
//         second_highest_index = find_highest_index(0, highest_index as usize, nums);
//         assert_ne!(second_highest_index, -1);
//         nums[highest_index as usize] + nums[second_highest_index as usize] * 10
//     } else {
//         nums[highest_index as usize] * 10 + nums[second_highest_index as usize]
//     }
// }

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
