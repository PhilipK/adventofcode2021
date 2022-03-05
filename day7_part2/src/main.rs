use std::{collections::HashMap, time::SystemTime};

fn main() {
    let before = SystemTime::now();
    let input = include_str!("input.txt");
    let mut input = parse_input(input);
    let best_position = calculate_best_position(&mut input);
    println!("{} in {:?}", best_position, before.elapsed());
}

fn parse_input(input: &str) -> Vec<u64> {
    input
        .split(",")
        .into_iter()
        .map(|s| s.parse::<u64>().unwrap())
        .collect()
}

fn calculate_best_position(input: &mut Vec<u64>) -> u64 {
    let max = input.iter().max().unwrap().clone();
    let possible_numbers = 0..max;
    // let mut best_number = last;
    let mut best_sum = u64::MAX;
    let mut factorial = MappedRecursiveFactorialCalculator::default();
    for i in possible_numbers {
        let cur_sum = get_total_fuel(&input, i, &mut factorial);
        if cur_sum < best_sum {
            // best_number = *i;
            best_sum = cur_sum
        }
    }
    return best_sum;
}

fn get_total_fuel<T: HasFactiorial>(input: &Vec<u64>, target: u64, calc: &mut T) -> u64 {
    let mut sum = 0;
    for i in input {
        let steps = (*i as i64 - target as i64).abs() as u64;
        sum += calc.factorial(steps);
    }
    return sum as u64;
}

trait HasFactiorial {
    fn factorial(&mut self, input: u64) -> u64;
}

#[derive(Default)]
struct FactorialCalculator {}

impl HasFactiorial for FactorialCalculator {
    fn factorial(&mut self, input: u64) -> u64 {
        let mut sum = 0;
        for i in 0..=input {
            sum += i;
        }
        return sum;
    }
}

#[derive(Default)]
struct MappedFactorialCalculator {
    map: HashMap<u64, u64>,
}

impl HasFactiorial for MappedFactorialCalculator {
    fn factorial(&mut self, input: u64) -> u64 {
        match self.map.get(&input) {
            Some(x) => return *x,
            None => {}
        }
        let mut sum = 0;
        for i in 0..=input {
            sum += i;
        }
        self.map.insert(input, sum);
        return sum;
    }
}

#[derive(Default)]
struct MappedRecursiveFactorialCalculator {
    map: HashMap<u64, u64>,
}

impl HasFactiorial for MappedRecursiveFactorialCalculator {
    fn factorial(&mut self, input: u64) -> u64 {
        match self.map.get(&input) {
            Some(x) => return *x,
            None => {}
        }
        let new_result = match input {
            0 => 0,
            1 => 1,
            2 => 3,
            3 => 6,
            x => self.factorial(x - 1) + x,
        };
        self.map.insert(input, new_result);
        return new_result;
    }
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
    fn can_calculate_fuel() {
        let mut factorial = MappedFactorialCalculator::default();
        let result = get_total_fuel(&mut vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14], 2, &mut factorial);
        assert_eq!(result, 206);
    }

    #[test]
    fn can_calculate_best_position() {
        let result = calculate_best_position(&mut vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14]);
        assert_eq!(result, 168);
    }

    #[test]
    fn factorial_test() {
        let mut factorial = FactorialCalculator::default();
        assert_eq!(0, factorial.factorial(0));
        assert_eq!(1, factorial.factorial(1));
        assert_eq!(3, factorial.factorial(2));
        assert_eq!(6, factorial.factorial(3));
    }

    #[test]
    fn factorial_test_mapped() {
        let mut factorial = MappedFactorialCalculator::default();
        assert_eq!(0, factorial.factorial(0));
        assert_eq!(1, factorial.factorial(1));
        assert_eq!(3, factorial.factorial(2));
        assert_eq!(6, factorial.factorial(3));
    }

    #[test]

    fn factorial_test_mapped_recursive() {
        let mut factorial = MappedRecursiveFactorialCalculator::default();
        assert_eq!(0, factorial.factorial(0));
        assert_eq!(1, factorial.factorial(1));
        assert_eq!(3, factorial.factorial(2));
        assert_eq!(6, factorial.factorial(3));
    }
}
