use std::env;

fn main() {
    let arg = env::args().nth(1).expect("Usage: primer <number>");
    let n: u64 = arg.parse().expect("Invalid whole number");

    println!("PRIME? {:?}", n);
}
