fn main() {
    let input = include_str!("input.txt");
    let directions = input.split("\n").into_iter().filter(|l| !l.is_empty()).map(str_to_direction);
    let mut horizontal = 0;
    let mut depth = 0;

    for direction in directions {
        match direction {
            Direction::Forward(x) => horizontal += x,
            Direction::Down(x) => depth += x,
            Direction::Up(x) => depth -= x,
        }
    }
    println!("{}", horizontal * depth);
}

fn str_to_direction<N: AsRef<str>>(input: N) -> Direction {
    let str = input.as_ref();
    let mut split = str.split(" ");
    let type_str = split.next().unwrap();
    let number_str = split.next().unwrap();
    let number = number_str.parse::<i32>().unwrap();
    match type_str {
        "forward" => Direction::Forward(number),
        "down" => Direction::Down(number),
        "up" => Direction::Up(number),
        _ => panic!("Unkown format"),
    }
}

enum Direction {
    Forward(i32),
    Down(i32),
    Up(i32),
}
