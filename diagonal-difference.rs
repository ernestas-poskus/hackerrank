use std::io;

fn main() {
    let mut dim = String::new();
    io::stdin().read_line(&mut dim).expect("read error");
    let dim: i32 = dim.trim().parse::<i32>().unwrap_or(0);

    let mut matrix: Vec<Vec<i32>> = Vec::new();
    for _ in 0..dim {
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).expect("read error");

        matrix.push(
            buf.split_whitespace()
                .map(|i| i.parse::<i32>().unwrap_or(0))
                .collect::<Vec<i32>>(),
        );
    }

    let mut prime: i32 = 0;
    let mut secon: i32 = 0;

    for (i, line) in matrix.iter().enumerate() {
        prime += line[i];
        secon += line[line.len() - 1 - i];
    }

    println!("{}", (prime - secon).abs());
}
