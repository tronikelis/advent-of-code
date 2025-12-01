use std::{
    env::args,
    fs::File,
    io::{BufRead, BufReader},
    os,
};

#[derive(Debug)]
enum DialDirection {
    L,
    R,
}

struct Dial {
    current: usize,
    password: usize,
}

impl Dial {
    fn new() -> Self {
        Self {
            current: 50,
            password: 0,
        }
    }

    fn turn(&mut self, dir: DialDirection, mut amount: usize) {
        match dir {
            DialDirection::L => {
                let result = self.current as isize - amount as isize;
                if result < 0 {
                    self.current = match (result % 100) + 100 {
                        100 => 0 as usize,
                        v => v as usize,
                    }
                } else {
                    self.current = result as usize;
                }
            }
            DialDirection::R => self.current = (self.current + amount) % 100,
        };

        self.check_password();
    }

    fn check_password(&mut self) {
        if self.current == 0 {
            self.password += 1;
        }
    }
}

fn main() {
    let filename = args().collect::<Vec<_>>();
    let filename = filename.get(1).expect("filename provided");
    let file = File::open(filename).expect("file opens");
    let mut file = BufReader::new(file);

    let mut dial = Dial::new();

    let mut buf = String::new();
    loop {
        buf.clear();
        if file.read_line(&mut buf).unwrap() == 0 {
            break;
        }

        let mut chars = buf.chars();

        let Some(direction) = chars.next() else {
            continue;
        };

        let direction = match direction {
            'L' => DialDirection::L,
            'R' => DialDirection::R,
            v => continue,
        };

        let num = chars.filter(|v| v.is_ascii_digit()).collect::<String>();

        dial.turn(direction, num.parse().unwrap());
    }

    println!("{}", dial.password);
}
