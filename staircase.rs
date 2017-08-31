use std::io;

fn main() {
    let mut dim = String::new();
    io::stdin().read_line(&mut dim).expect("read error");
    let dim: usize = dim.trim().parse::<usize>().unwrap_or(0) + 1;

    for i in 1..dim {
        println!("{}{}", " ".repeat(dim - 1 - i), "#".repeat(i))
    }
}
