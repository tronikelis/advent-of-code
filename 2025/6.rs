use std::{
    env::args,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, PartialEq)]
enum Action {
    Mul,
    Plus,
}

fn transpose(from: Vec<Vec<String>>) -> Vec<Vec<String>> {
    if from.len() == 0 {
        return from;
    }

    let mut result = Vec::<Vec<String>>::new();

    let first = &from[0];
    for i in 0..first.len() {
        let mut row = Vec::<String>::new();
        for j in 0..from.len() {
            row.push(from[j][i].clone());
        }
        result.push(row);
    }

    result
}

fn pad(target: &mut Vec<String>) {
    let max_len = target.iter().fold(
        0,
        |acc, curr| {
            if acc < curr.len() {
                curr.len()
            } else {
                acc
            }
        },
    );

    for x in target {
        let pad_amount = max_len - x.len();
        if pad_amount != 0 {
            *x = format!("{}{}", " ".repeat(pad_amount), *x);
        }
    }
}

fn main() {
    let filename = args().collect::<Vec<_>>();
    let filename = filename.get(1).expect("filename provided");
    let file = File::open(filename).expect("file opens");
    let mut file = BufReader::new(file);

    let mut lines = Vec::<Vec<String>>::new();
    let mut actions: Option<Vec<Action>> = None;

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

        let first_char = string.chars().next();
        let Some(first_char) = first_char else {
            panic!("none string char encountered");
        };

        if first_char.is_ascii_digit() {
            let parsed = string
                .split(" ")
                .filter(|v| v.trim_ascii().len() != 0)
                .map(|v| v.to_string())
                .collect::<Vec<_>>();

            lines.push(parsed);
        } else {
            assert_eq!(actions, None);

            let parsed = string
                .split(" ")
                .filter(|v| v.trim_ascii().len() != 0)
                .map(|v| match v {
                    "*" => Action::Mul,
                    "+" => Action::Plus,
                    _ => panic!("unknown char"),
                })
                .collect::<Vec<_>>();

            actions = Some(parsed);
        }
    }

    // dbg!(&actions, &lines);

    // pad the strings
    let mut lines = transpose(lines);
    for x in &mut lines {
        pad(x);
    }

    dbg!(&actions, &lines);

    let mut acc: usize = 0;
    for (line_i, line) in lines.into_iter().enumerate() {
        let mut iter: usize = 0;

        let line_0_len = line[0].len();
        for i in 0..line_0_len {
            let mut iter2: usize = 0;
            let mut pow_counter = 0;
            for j in (0..line.len()).rev() {
                let num = line[j].chars().collect::<Vec<_>>()[i];
                let Some(num) = num.to_digit(10) else {
                    continue;
                };

                dbg!(num, pow_counter);
                iter2 += (num * 10_u32.pow(pow_counter)) as usize;
                pow_counter += 1;
            }
            dbg!(iter2);
            match actions.as_ref().unwrap()[line_i] {
                Action::Mul => iter *= iter2,
                Action::Plus => iter += iter2,
            }
        }

        acc += iter;
    }

    // let outer_len = lines[0].len();
    // let mut acc: usize = 0;
    //
    // for i in 0..outer_len {
    //     let op = actions.as_ref().unwrap().get(i).unwrap();
    //     let mut result: Option<usize> = None;
    //
    //     for j in 0..lines.len() {
    //         let num = lines[j][i];
    //         dbg!(op, num);
    //         match op {
    //             Action::Mul => result = Some(result.unwrap_or(1) * num),
    //             Action::Plus => result = Some(result.unwrap_or(0) + num),
    //         }
    //     }
    //
    //     acc += result.unwrap_or(0);
    // }

    println!("{}", acc);
}
