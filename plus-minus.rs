use std::io;

fn main() {
    let mut dim = String::new();
    io::stdin().read_line(&mut dim).expect("read error");
    let _dim: i32 = dim.trim().parse::<i32>().unwrap_or(0);

    let mut arr = String::new();
    io::stdin().read_line(&mut arr).expect("read error");

    let arr: Vec<i32> = arr.split_whitespace()
        .map(|i| i.parse::<i32>().unwrap_or(0))
        .collect();

    let size: f32 = arr.len() as f32;
    let (negative, rest): (Vec<i32>, Vec<i32>) = arr.iter().partition(|&n| n < &0);
    let (zeros, positive): (Vec<i32>, Vec<i32>) = rest.iter().partition(|&n| n == &0);

    println!("{:.6}", positive.len() as f32 / size);
    println!("{:.6}", negative.len() as f32 / size);
    println!("{:.6}", zeros.len() as f32 / size);
}
