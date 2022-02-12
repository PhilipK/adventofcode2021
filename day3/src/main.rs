use std::panic;

fn main() {
    let result = calculate_gamme_times_epsilon(include_str!("input.txt"));
    println!("{}", result);
}

fn calculate_gamme_times_epsilon<N: AsRef<str>>(input: N) -> usize {
    let input = input.as_ref();
    let lines = input.split("\n").filter(|s| !s.is_empty());
    let number_of_bits = lines.clone().next().unwrap().len();
    let mut epsilon_string = String::new();
    let mut gamma_string  = String::new();
    for index in 0..number_of_bits{
        let mut ones = 0;
        let mut zeros = 0;
        for line in lines.clone() {
            match &line[index..index+1] {
                "0" => zeros += 1,
                "1" => ones += 1,
                _=> panic!("non binary")
            }
        }
        if ones > zeros{
            gamma_string.push('1');
            epsilon_string.push('0');
        }else{
            gamma_string.push('0');
            epsilon_string.push('1');
        }
    }
    let epsilon = usize::from_str_radix(&epsilon_string, 2).unwrap();
    let gamma = usize::from_str_radix(&gamma_string, 2).unwrap();
    return epsilon*gamma;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let input = include_str!("testinput.txt");
        let result = calculate_gamme_times_epsilon(input);
        assert_eq!(result, 198);
    }
}
