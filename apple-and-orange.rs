use std::io;

fn main() {
    let mut data: Vec<Vec<i32>> = Vec::new();

    for _ in 0..5 {
        let mut buff = String::new();
        io::stdin().read_line(&mut buff).expect("read error");

        data.push(
            buff.split_whitespace()
                .map(|i| i.parse::<i32>().unwrap_or(0))
                .collect(),
        );
    }

    println!(
        "{}",
        data[3]
            .iter()
            .filter(|&n| {
                n + data[1][0] >= data[0][0] && n + data[1][0] <= data[0][1]
            })
            .count()
    );

    println!(
        "{}",
        data[4]
            .iter()
            .filter(|&n| {
                n + data[1][1] >= data[0][0] && n + data[1][1] <= data[0][1]
            })
            .count()
    );
}
