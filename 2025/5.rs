use std::{
    env::args,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
struct Data {
    ranges: Vec<(usize, usize)>,
    ids: Vec<usize>,
}

impl Data {
    fn new() -> Self {
        Self {
            ranges: Vec::new(),
            ids: Vec::new(),
        }
    }

    fn id_count_from_ranges(&self) -> usize {
        let mut acc = 0;

        for v in &self.ranges {
            acc += v.1 - v.0 + 1;
        }

        acc
    }

    fn id_in_range(id: usize, range: (usize, usize)) -> bool {
        id >= range.0 && id <= range.1
    }

    fn fresh_id_count(&self) -> usize {
        let mut count = 0;
        for id in &self.ids {
            if self.ranges.iter().any(|v| Self::id_in_range(*id, *v)) {
                count += 1;
            }
        }
        count
    }

    fn merge_ranges(&mut self) {
        loop {
            let mut done_swap = false;

            let mut i: isize = -1;
            while i + 1 < self.ranges.len() as isize - 1 {
                i += 1;
                let i = i as usize;

                let mut j = i;
                while j + 1 < self.ranges.len() {
                    j += 1;

                    let first = self.ranges[i as usize];
                    let second = self.ranges[j as usize];

                    // first range full overlap, remove second
                    if first.1 >= second.1 && first.0 <= second.0 {
                        self.ranges.swap_remove(j);
                        done_swap = true;
                        continue;
                    }

                    // second range full overlap, remove first
                    if first.1 <= second.1 && first.0 >= second.0 {
                        self.ranges.swap_remove(i);
                        done_swap = true;
                        continue;
                    }

                    // extend right
                    if Self::id_in_range(second.0, first) {
                        self.ranges[i as usize].1 = second.1;
                        self.ranges.swap_remove(j);
                        done_swap = true;
                        continue;
                    }

                    // extend left
                    if Self::id_in_range(second.1, first) {
                        self.ranges[i as usize].0 = second.0;
                        self.ranges.swap_remove(j);
                        done_swap = true;
                        continue;
                    }
                }
            }

            if !done_swap {
                break;
            }
        }
    }

    fn add_range(&mut self, range: (usize, usize)) {
        self.ranges.push(range);
    }

    fn add_id(&mut self, id: usize) {
        self.ids.push(id);
    }
}

fn main() {
    let filename = args().collect::<Vec<_>>();
    let filename = filename.get(1).expect("filename provided");
    let file = File::open(filename).expect("file opens");
    let mut file = BufReader::new(file);

    let mut blank_line = false;

    let mut data = Data::new();

    let mut buf = String::new();
    loop {
        buf.clear();
        if file.read_line(&mut buf).unwrap() == 0 {
            break;
        }

        let string = buf.trim_ascii();
        if string.len() == 0 {
            blank_line = true;
            continue;
        }

        if blank_line {
            data.add_id(string.parse().unwrap());
        } else {
            let mut it = string.split("-");
            data.add_range((
                it.next().unwrap().parse().unwrap(),
                it.next().unwrap().parse().unwrap(),
            ));
        }
    }

    dbg!(&data);
    data.merge_ranges();
    dbg!(&data);

    println!("fresh id count: {}", data.fresh_id_count());
    println!("id count from ranges: {}", data.id_count_from_ranges());
}
