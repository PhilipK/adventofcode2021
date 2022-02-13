use core::num;

fn main() {
    let input = include_str!("input.txt");
    let value = calculate_lines(input);
    println!("{}", value);
}

fn calculate_lines<N: AsRef<str>>(input: N) -> u32 {
    let board = get_board(input);
    board.number_of_spots_above(2) as u32
}

fn get_board<N: AsRef<str>>(input: N) -> Board {
    let mut lines = parse_lines(input);
    let (max_x, max_y) = get_max_dimentions(&lines);
    lines.retain(|line| line.is_horizontal() || line.is_vertical());
    let mut board = Board::new(max_x as usize + 1, max_y as usize + 1);
    for line in lines {
        board.mark_line(&line);
    }
    board
}

fn get_max_dimentions(lines: &Vec<Line>) -> (u32, u32) {
    let mut max_x = 0;
    let mut max_y = 0;
    for line in lines {
        if line.to.0 > max_x {
            max_x = line.to.0;
        }
        if line.from.0 > max_x {
            max_x = line.from.0;
        }
        if line.to.1 > max_y {
            max_y = line.to.1;
        }
        if line.from.1 > max_y {
            max_y = line.from.1;
        }
    }

    (max_x, max_y)
}

fn parse_lines<N: AsRef<str>>(input: N) -> Vec<Line> {
    let input = input.as_ref();
    input
        .lines()
        .filter(|s| !s.is_empty())
        .map(string_line_to_line)
        .collect()
}

fn string_line_to_line<N: AsRef<str>>(input: N) -> Line {
    let line = input.as_ref();

    let mut split = line.split(" -> ").into_iter().map(|numbers| {
        let mut numbers = numbers
            .split(",")
            .map(|number_string| number_string.parse::<u32>().unwrap());
        (numbers.next().unwrap(), numbers.next().unwrap())
    });
    let from = split.next().unwrap();
    let to = split.next().unwrap();
    Line { from, to }
}

struct Board {
    board: Vec<u32>,
    width: usize,
}

impl Board {
    fn new(width: usize, height: usize) -> Self {
        Board {
            board: vec![0; width * height],
            width,
        }
    }

    fn mark_spot(&mut self, x: u32, y: u32) {
        self.board[self.width * y as usize + x as usize] += 1;
    }

    fn mark_line(&mut self, line: &Line) {
        if line.is_horizontal() {
            if line.from.0 > line.to.0 {
                for i in line.to.0..=line.from.0 {
                    self.mark_spot(i, line.from.1);
                }
            } else {
                for i in line.from.0..=line.to.0 {
                    self.mark_spot(i, line.from.1);
                }
            }
        }

        if line.is_vertical() {
            if line.from.1 > line.to.1 {
                for i in line.to.1..=line.from.1 {
                    self.mark_spot(line.from.0, i);
                }
            } else {
                for i in line.from.1..=line.to.1 {
                    self.mark_spot(line.from.0, i);
                }
            }
        }
    }

    fn number_of_spots_above(&self, target: u32) -> usize {
        self.board.iter().filter(|x| **x >= target).count()
    }

    fn to_diagram(&self) -> String {
        let mut res = String::new();
        self.board.chunks(self.width).for_each(|line| {
            for x in line {
                res.push_str(format!("{}", x).as_str());
            }
            res.push_str("\n");
        });

        res
    }
}

#[derive(Clone)]
struct Line {
    from: (u32, u32),
    to: (u32, u32),
}

impl Line {
    fn is_horizontal(&self) -> bool {
        self.from.1 == self.to.1
    }

    fn is_vertical(&self) -> bool {
        self.from.0 == self.to.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_lines() {
        let input = include_str!("testinput.txt");
        let lines = parse_lines(input);
        assert_eq!(lines.len(), 10);
        assert_eq!(lines[0].from, (0, 9));
        assert_eq!(lines[0].to, (5, 9));
    }

    #[test]
    fn test_input() {
        let input = include_str!("testinput.txt");
        let actual = calculate_lines(input);

        assert_eq!(actual, 5);
    }

    #[test]
    fn show_diagram() {
        let input = include_str!("testinput.txt");
        let board = get_board(input);
        println!("{}", board.to_diagram());
    }

    #[test]
    fn can_mark_line() {
        let mut board = Board::new(5, 5);

        let line = Line {
            from: (0, 1),
            to: (4, 1),
        };
        board.mark_line(&line);
        board.mark_line(&line);
        let total = board.number_of_spots_above(2);

        assert_eq!(total, 5);
    }

    #[test]
    fn can_mark_line_backwards() {
        let mut board = Board::new(5, 5);

        let line = Line {
            from: (4, 1),
            to: (0, 1),
        };
        board.mark_line(&line);
        board.mark_line(&line);
        let total = board.number_of_spots_above(2);

        assert_eq!(total, 5);
    }

    #[test]
    fn is_horizontal() {
        let line = Line {
            from: (0, 1),
            to: (4, 1),
        };

        assert_eq!(false, line.is_vertical());
        assert_eq!(true, line.is_horizontal());
    }

    #[test]
    fn is_vertical() {
        let line = Line {
            from: (0, 0),
            to: (3, 0),
        };

        assert_eq!(true, line.is_horizontal());
        assert_eq!(false, line.is_vertical());
    }

    #[test]
    fn can_mark_line_vert() {
        let mut board = Board::new(5, 5);

        let line = Line {
            from: (1, 0),
            to: (1, 4),
        };
        board.mark_line(&line);
        board.mark_line(&line);
        let total = board.number_of_spots_above(2);

        assert_eq!(total, 5);
    }
}
