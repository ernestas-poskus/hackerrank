use std::io;

fn main() {
    let mut time = String::new();
    io::stdin().read_line(&mut time).expect("read error");
    let (time, format) = time.trim().split_at(8);
    let mut time: Vec<u8> = time.split(":")
        .map(|t| t.parse::<u8>().unwrap_or(0))
        .collect();
    if format == "PM" && time[0] < 12 {
        time[0] += 12;
    }
    if format == "AM" && time[0] == 12 {
        time[0] = 0;
    }

    println!("{:02}:{:02}:{:02}", time[0], time[1], time[2]);
}
