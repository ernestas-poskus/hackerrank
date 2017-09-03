use std::io;

fn main() {
    let mut l = String::new();
    io::stdin().read_line(&mut l).expect("read error");
    let l: Vec<usize> = l.split_whitespace()
        .map(|i| i.parse::<usize>().unwrap_or(0))
        .collect();

    if l[1] > l[3] && (l[2] - l[0]) % (l[1] - l[3]) == 0 {
        println!("YES");
    } else {
        println!("NO");
    }
}
