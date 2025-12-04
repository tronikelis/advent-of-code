use std::{
    env::args,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, Clone, Copy)]
enum Pos {
    Roll,
    Empty,
    Occupied,
}

#[derive(Debug)]
struct Grid(Vec<Vec<Pos>>);

impl Grid {
    fn check_pos(pos: Pos, if_roll: &mut usize) {
        match pos {
            Pos::Roll => *if_roll += 1,
            Pos::Empty => {}
            Pos::Occupied => {}
        }
    }

    fn check_3_adj(&self, y: usize, x: usize, if_roll: &mut usize) {
        for i in 0..3 {
            let offset = x as isize + i as isize - 1;
            if offset >= 0 && offset < self.0[y].len() as isize {
                Self::check_pos(self.0[y][offset as usize], if_roll);
            }
        }
    }

    fn access_count(&mut self) -> usize {
        let mut count = 0;

        for y in 0..self.0.len() {
            for x in 0..self.0[y].len() {
                let Pos::Roll = self.0[y][x] else {
                    print!(".");
                    continue;
                };

                let mut rolls = 0;

                // check 3 adj positions top
                let mut top_row = (y as isize) - 1;
                if top_row >= 0 {
                    self.check_3_adj(top_row as usize, x, &mut rolls);
                }

                // check left
                let left_row = (x as isize) - 1;
                if left_row >= 0 {
                    Self::check_pos(self.0[y][left_row as usize], &mut rolls);
                }

                // check right
                let right_row = x + 1;
                if right_row < self.0[y].len() {
                    Self::check_pos(self.0[y][right_row as usize], &mut rolls);
                }

                // check 3 adj positions bottom
                let mut bottom_row = y + 1;
                if bottom_row < self.0.len() {
                    self.check_3_adj(bottom_row, x, &mut rolls);
                }

                if rolls < 4 {
                    count += 1;
                    print!("x");
                } else {
                    print!("@");
                }
            }
            println!();
        }

        count
    }
}

fn main() {
    let filename = args().collect::<Vec<_>>();
    let filename = filename.get(1).expect("filename provided");
    let file = File::open(filename).expect("file opens");
    let mut file = BufReader::new(file);

    let mut acc: usize = 0;

    let mut grid = Vec::new();

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
            .map(|v| match v {
                '@' => Pos::Roll,
                '.' => Pos::Empty,
                _ => panic!("unknown char"),
            })
            .collect::<Vec<_>>();

        grid.push(nums);
    }

    let mut grid = Grid(grid);
    println!("{}", grid.access_count());
}
