use std::io;

pub fn get_move() -> usize{
    let mut m = String::new();
    io::stdin()
        .read_line(&mut m)
        .expect("Failed to read line");
    m.pop();
    if m.len() == 1 {
        let c = m.chars().nth(0);
        match c {
            Some('w') | Some('W') => return 0,
            Some('a') | Some('A') => return 1,
            Some('s') | Some('S') => return 2,
            Some('d') | Some('D') => return 3,
            Some('x') | Some('X') => return 4,
            Some(_) => return 5,
            None => return 5,
        }
    }
    println!("Input is: {m} !");
    5
}