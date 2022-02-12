use std::time::SystemTime;

fn main() {
    let input = include_str!("input.txt");
    let now = SystemTime::now();
    let res = calculate_bingo_score(input);
    println!("time {:?}", now.elapsed());
    println!("{}", res);
}

fn calculate_bingo_score<N: AsRef<str>>(input: N) -> usize {
    let (numbers, mut boards) = parse_input(input);
    for (i, number) in numbers.iter().enumerate() {
        for board in &mut boards {
            board.mark_number(*number);
            if board.is_done() {
                return board.unmarked_sum() * (*number as usize);
            }
        }
    }
    return 0;
}

fn parse_input<N: AsRef<str>>(input: N) -> (Vec<u32>, Vec<Board<25, 5>>) {
    let input = input.as_ref();
    let mut lines = input.lines().into_iter();

    let number_line = lines.next().unwrap();
    let numbers = number_line
        .split(",")
        .map(|number_str| number_str.parse::<u32>().unwrap())
        .collect();
    let mut boards = vec![];

    const WIDTH: usize = 5;
    const SIZE: usize = WIDTH * WIDTH;

    //empty line seperator
    while lines.next().is_some() {
        let mut numbers = [0; WIDTH * WIDTH];

        for row in 0..WIDTH {
            let line = lines.next().unwrap();
            let iter = line
                .split(" ")
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<u32>().unwrap());
            for (i, number) in iter.enumerate() {
                numbers[row * WIDTH + i] = number;
            }
        }
        let board = Board::<SIZE, WIDTH>::new(numbers);
        boards.push(board);
    }

    (numbers, boards)
}

struct Board<const N: usize, const Width: usize> {
    numbers: [u32; N],
    marked: [bool; N],
}

impl<const Width: usize, const N: usize> Board<N, Width> {
    pub fn new(input: [u32; N]) -> Self {
        Board {
            numbers: input,
            marked: [false; N],
        }
    }

    pub fn mark_number(&mut self, input: u32) {
        for (i, number) in self.numbers.iter().enumerate() {
            if *number == input {
                self.marked[i] = true;
            }
        }
    }

    pub fn is_done(&self) -> bool {
        for row in 0..Width {
            if self.row_done(row) {
                return true;
            }
        }
        for column in 0..Width {
            if self.column_done(column) {
                return true;
            }
        }
        return false;
    }

    fn column_done(&self, column: usize) -> bool {
        let mut index = column;
        while index < N {
            if !self.marked[index] {
                return false;
            }
            index += Width;
        }
        return true;
    }

    fn row_done(&self, row: usize) -> bool {
        let start = row * Width;
        for i in start..start + Width {
            if !self.marked[i] {
                return false;
            }
        }
        return true;
    }

    fn unmarked_sum(&self) -> usize {
        let mut sum = 0;
        self.numbers
            .iter()
            .enumerate()
            .filter(|(i, _)| !self.marked[*i])
            .for_each(|(_i, number)| {
                sum += *number as usize;
            });
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_input() {
        let input = include_str!("testinput.txt");
        parse_input(input);
    }

    #[test]
    fn test_input() {
        let input = include_str!("testinput.txt");
        let result = calculate_bingo_score(input);
        assert_eq!(result, 4512);
    }
}
