use std::{
    env::args,
    fs::File,
    io::{BufRead, BufReader},
};

fn parse_invalid_ids(first: usize, second: usize) -> Vec<usize> {
    let mut invalid_ids = Vec::new();

    // part 1
    // let mut i = first;
    // while i <= second {
    //     let i_str = i.to_string();
    //     if i_str.len() % 2 != 0 {
    //         i += 1;
    //         continue;
    //     }
    //
    //     let mid = i_str.len() / 2;
    //     let first_half = &i_str[0..mid];
    //     let second_half = &i_str[mid..i_str.len()];
    //     let combined = format!("{}{}", first_half, first_half).parse().unwrap();
    //
    //     if combined >= first && combined <= second {
    //         invalid_ids.push(combined);
    //
    //         // skip to next possible half
    //         let mut first_half_parsed: usize = first_half.parse().unwrap();
    //         first_half_parsed += 1;
    //
    //         let new_i = format!("{}{}", first_half_parsed, second_half)
    //             .parse()
    //             .unwrap();
    //
    //         i = new_i;
    //     } else {
    //         i += 1;
    //     }
    // }

    for i in first..=second {
        let num = i.to_string();

        let mut curr = &num[0..1];
        let mut index = 1;
        loop {
            // lose
            if curr.len() > num.len() / 2 {
                break;
            }

            // win
            if index == num.len() {
                // dbg!(i);
                invalid_ids.push(i);
                break;
            }

            // oob index, lose
            if index + curr.len() > num.len() {
                break;
            }

            let next = &num[index..(index + curr.len())];
            if curr == next {
                index += curr.len();
                continue;
            }
            index += 1;
            curr = &num[0..index];
        }
    }

    invalid_ids
}

fn main() {
    let filename = args().collect::<Vec<_>>();
    let filename = filename.get(1).expect("filename provided");
    let file = File::open(filename).expect("file opens");
    let mut file = BufReader::new(file);

    let mut acc: usize = 0;

    let mut buf = Vec::new();
    loop {
        buf.clear();
        if file.read_until(b',', &mut buf).unwrap() == 0 {
            break;
        }

        let mut string = String::from_utf8(buf.clone()).unwrap();
        let mut string_ref = string.trim_ascii();

        if string_ref.len() == 0 {
            continue;
        }

        if string_ref.ends_with(',') {
            string_ref = &string_ref[0..string_ref.len() - 1];
        }

        let mut minus_iter = string_ref.split('-');
        let first_digit: usize = minus_iter.next().unwrap().parse().unwrap();
        let second_digit: usize = minus_iter.next().unwrap().parse().unwrap();

        for v in parse_invalid_ids(first_digit, second_digit) {
            acc += v;
        }
    }

    println!("{}", acc);
}
