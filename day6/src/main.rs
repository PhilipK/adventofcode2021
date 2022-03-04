fn main() {
    let input = include_str!("input.txt");
    let mut fish = parse_input(input);
    let total = calculate_total_fish(&mut fish, 80);
    println!("{}",total);
}

fn parse_input(input:&str) -> Vec<u8>{
    input.split(",").into_iter().map(|s| s.parse::<u8>().unwrap()).collect()
}

fn calculate_total_fish(input: &mut Vec<u8>, days: u32) -> usize {
    let mut remaining_days = days;
    while remaining_days > 0 {
        let new_fish = decrement_day(input);
        let mut new_fish_array :Vec<u8> = vec![8 ;new_fish];
        input.append(&mut new_fish_array);
        remaining_days -= 1;
    }
    return input.len();
}

//Decrements the fish age, and returns how many new fish to spawn
fn decrement_day(input: &mut Vec<u8>) -> usize {
    let mut new_fish = 0;
    for fish in input {
        if *fish > 0 {
            *fish -= 1;
        } else {
            *fish = 6;
            new_fish += 1;
        }
    }
    return new_fish;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_input() {
        let result = parse_input("3,4,3,1,2");
        assert_eq!(result, vec![3, 4, 3, 1, 2]);
    }

    #[test]
    fn can_calculate_fish() {
        let result = calculate_total_fish(&mut vec![3, 4, 3, 1, 2], 80);
        assert_eq!(result, 5934);
    }
}
