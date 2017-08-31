use std::io;

fn main() {
    let mut numbers = String::new();
    io::stdin().read_line(&mut numbers).expect("read error");
    let numbers: Vec<i64> = numbers
        .split_whitespace()
        .map(|i| i.parse::<i64>().unwrap_or(0))
        .collect();

    let mut total: i64 = 0;

    let mut min_val: i64 = numbers[0];
    let mut max_val: i64 = 0;

    for number in numbers {
        total += number;
        if number > max_val {
            max_val = number;
        }
        if number < min_val {
            min_val = number;
        }
    }

    println!("{} {}", total - max_val, total - min_val);
}
