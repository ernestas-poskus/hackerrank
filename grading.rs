use std::io;

fn main() {
    let mut num = String::new();
    io::stdin().read_line(&mut num).expect("read error");
    let num: usize = num.trim().parse::<usize>().unwrap_or(0);

    for _n in 0..num {
        let mut buff = String::new();
        io::stdin().read_line(&mut buff).expect("read error");
        let mut grade: u8 = buff.trim().parse::<u8>().unwrap_or(0);
        if grade > 37 {
            for ng in grade..101 {
                if ng % 5 == 0 {
                    if (ng - grade) < 3 {
                        grade = ng;
                    }
                    break;
                }
            }
        }
        println!("{}", grade);
    }
}
