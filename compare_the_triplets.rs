use std::io;

fn main() {
    let mut a = String::new();
    let mut b = String::new();
    io::stdin().read_line(&mut a).expect("read error");
    io::stdin().read_line(&mut b).expect("read error");

    let a: Vec<i32> = a.split_whitespace()
        .map(|i| i.parse::<i32>().unwrap_or(0))
        .collect();
    let b: Vec<i32> = b.split_whitespace()
        .map(|i| i.parse::<i32>().unwrap_or(0))
        .collect();

    let mut result = (0, 0);

    for (i, a_num) in a.iter().enumerate() {
        let b_num = b[i];
        if a_num == &b_num {
            continue;
        } else if a_num > &b_num {
            result.0 += 1;
        } else {
            result.1 += 1;
        }
    }
    println!("{} {}", result.0, result.1);
}
