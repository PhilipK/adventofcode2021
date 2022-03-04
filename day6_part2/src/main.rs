fn main() {
    let input = include_str!("input.txt");
    let mut fish = parse_input(input);
    let total = calculate_total_fish(&mut fish, 256);
    println!("{}", total);
}

fn parse_input(input: &str) -> Vec<u8> {
    input
        .split(",")
        .into_iter()
        .map(|s| s.parse::<u8>().unwrap())
        .collect()
}

fn calculate_total_fish(input: &mut Vec<u8>, days: u32) -> usize {
    let mut fish_of_ages: [u64; 9] = [0; 9];

    for fish in input {
        fish_of_ages[*fish as usize] = fish_of_ages[*fish as usize] + 1;
    }

    let mut remaining_days = days;
    while  remaining_days > 0{
        decrement_day(&mut fish_of_ages);
        remaining_days -= 1;
    }



    return fish_of_ages.iter().sum::<u64>() as usize;
}

fn decrement_day(fish_of_ages: &mut [u64; 9]){
    let birthing_fish = fish_of_ages[0];
    fish_of_ages[0] = fish_of_ages[1];
    fish_of_ages[1] = fish_of_ages[2];
    fish_of_ages[2] = fish_of_ages[3];
    fish_of_ages[3] = fish_of_ages[4];
    fish_of_ages[4] = fish_of_ages[5];
    fish_of_ages[5] = fish_of_ages[6] ;
    fish_of_ages[6] = fish_of_ages[7] + birthing_fish;
    fish_of_ages[7] = fish_of_ages[8];
    fish_of_ages[8] = birthing_fish;
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
    fn can_calculate_fish_small() {
        let result = calculate_total_fish(&mut vec![3, 4, 3, 1, 2], 80);
        assert_eq!(result, 5934);
    }

    #[test]
    fn can_calculate_fish() {
        let result = calculate_total_fish(&mut vec![3, 4, 3, 1, 2], 256);
        assert_eq!(result, 26984457539);
    }
}
