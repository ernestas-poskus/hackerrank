use std::io;

fn main() {
    let mut num = String::new();
    let mut candles = String::new();
    io::stdin().read_line(&mut num).expect("read error");
    io::stdin().read_line(&mut candles).expect("read error");

    let _num: Vec<i32> = num.split_whitespace()
        .map(|i| i.parse::<i32>().unwrap_or(0))
        .collect();
    let candles: Vec<i32> = candles
        .split_whitespace()
        .map(|i| i.parse::<i32>().unwrap_or(0))
        .collect();

    let max = candles.iter().max().unwrap();
    println!("{}", candles.iter().filter(|&n| n == max).count());
}
