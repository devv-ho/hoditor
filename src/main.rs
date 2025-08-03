use std::io;

fn main() {
    let mut a = String::new();

    io::stdin().read_line(&mut a).expect("Failed to read");

    let a: u32 = a.trim().parse().expect("it has to be a number");
    println!("{a}");
}
