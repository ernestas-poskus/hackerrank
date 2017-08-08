use std::io::{self, Read};

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap_or(0);
    let mut lines = buffer.split("\n");
    println!("{}",
             lines
                 .nth(1)
                 .unwrap_or("")
                 .split(" ")
                 .map(|i| i.parse::<i32>().unwrap_or(0))
                 .sum::<i32>());
}
