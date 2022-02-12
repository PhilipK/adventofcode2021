fn main() {

    let input_str = include_str!("input.txt");
    let mut numbers = input_str.split("\n").into_iter().map(|line| line.parse::<i32>().unwrap());
    let mut result = 0;
    let mut prev = numbers.next().unwrap();
    for number in numbers {
        if number > prev {
            result += 1;
        }
        prev = number;
    }
    println!("{}",result);
}
