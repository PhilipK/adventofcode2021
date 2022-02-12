fn main() {
    let input_str = include_str!("input.txt");
    let numbers: Vec<i32> = input_str
        .split("\n")
        .into_iter()
        .map(|line| line.parse::<i32>().unwrap())
        .collect();
    let number_of_numbers = numbers.len();
    let mut sums = Vec::with_capacity(number_of_numbers);
    for i in 0..number_of_numbers - 2 {
        sums.push(numbers[i] + numbers[i + 1] + numbers[i + 2]);
    }
    let result = times_incremented(&sums);
    println!("{}", result);
}

fn times_incremented(numbers: &Vec<i32>) -> i32 {
    let mut iter = numbers.iter();
    let mut result = 0;
    let mut prev = iter.next().unwrap();
    for number in iter {
        if number > prev {
            result += 1;
        }
        prev = number;
    }
    result
}
