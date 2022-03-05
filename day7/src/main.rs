fn main() {
    let input = include_str!("input.txt");
    let mut input = parse_input(input);
    let best_position = calculate_best_position(&mut input);
    println!("{}", best_position);
}

fn parse_input(input: &str) -> Vec<u32> {
    input
        .split(",")
        .into_iter()
        .map(|s| s.parse::<u32>().unwrap())
        .collect()
}

fn calculate_best_position(input: &mut Vec<u32>) -> u32 {

    let mut possible_numbers = input.clone();
    //Sort to make it easy to dedupe
    possible_numbers.sort();
    //Remove duplicates
    possible_numbers.dedup();

    //Just start with assuming the last one is the best one
    let last = possible_numbers.pop().unwrap();
    // let mut best_number = last;
    let mut best_sum = get_total_distance(&input, last);

    for i in possible_numbers.iter() {
        let cur_sum = get_total_distance(&input, *i);
        if cur_sum < best_sum {
            // best_number = *i;
            best_sum = cur_sum
        }
    }
    return best_sum;
}

fn get_total_distance(input: &Vec<u32>, target: u32) -> u32 {
    let mut sum = 0;
    for i in input {
        sum += (*i as i32 - target as i32).abs();
    }
    return sum as u32;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_input() {
        let result = parse_input("16,1,2,0,4,2,7,1,2,14");
        assert_eq!(result, vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14]);
    }

    #[test]
    fn can_calculate_best_position() {
        let result = calculate_best_position(&mut vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14]);
        assert_eq!(result, 37);
    }
}
