use std::panic;

fn main() {
    let result = calculate_ratings_multiplies(include_str!("input.txt"));
    println!("{}", result);
}

fn calculate_ratings_multiplies<N: AsRef<str>>(input: N) -> usize {
    let input = input.as_ref();
    let lines = input.split("\n").filter(|s| !s.is_empty());

    let mut remaining: Vec<&str> = lines.clone().collect();
    filter_matching(&mut remaining, true);
    let oxygen = remaining[0];

    let mut remaining: Vec<&str> = lines.clone().collect();
    filter_matching(&mut remaining, false);
    let co2 = remaining[0];

    parse_binary_number(oxygen) * parse_binary_number(co2)
}

fn filter_matching(remaining: &mut Vec<&str>, search_for_most_common: bool) {
    let mut index = 0;
    while remaining.len() > 1 {
        let mut most_common = get_most_common_bit(&*remaining, index);
        if !search_for_most_common {
            most_common = most_common.flipped();
        }
        *remaining = remaining
            .iter()
            .filter(|line| line_has_bit(line, index, most_common))
            .map(|s| *s)
            .collect();
        index += 1;
    }
}

fn line_has_bit<N: AsRef<str>>(input: N, index: usize, common_bit: CommonBit) -> bool {
    let input = input.as_ref();
    match common_bit {
        CommonBit::One => &input[index..index + 1] == "1",
        CommonBit::Zero => &input[index..index + 1] == "0",
    }
}

fn parse_binary_number<N: AsRef<str>>(input: N) -> usize {
    let epsilon = usize::from_str_radix(input.as_ref(), 2).unwrap();
    epsilon
}

fn get_most_common_bit(input: &Vec<&str>, index: usize) -> CommonBit {
    let mut ones = 0;
    let mut zeros = 0;
    for line in input {
        match &line[index..index + 1] {
            "0" => zeros += 1,
            "1" => ones += 1,
            _ => panic!("non binary"),
        }
    }
    if ones >= zeros {
        CommonBit::One
    } else {
        CommonBit::Zero
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum CommonBit {
    One,
    Zero,
}

impl CommonBit {
    fn flipped(&self) -> Self {
        match self {
            CommonBit::One => CommonBit::Zero,
            CommonBit::Zero => CommonBit::One,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let input = include_str!("testinput.txt");
        let result = calculate_ratings_multiplies(input);
        assert_eq!(result, 230);
    }

    #[test]
    fn most_common_bit() {
        let input = vec!["001", "100", "001"];
        let res = get_most_common_bit(&input.clone(), 0);
        assert_eq!(res, CommonBit::Zero);

        let res = get_most_common_bit(&input, 2);
        assert_eq!(res, CommonBit::One);
    }
}
