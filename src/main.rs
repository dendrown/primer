/// primer:
/// A persisent prime producer for play!
///
/// Copyright (c) 2026 Dennis Drown
use std::env;
use std::process;
use std::process::ExitCode;
use nix::Error;

use primer::Context;

const EINVAL: i32 = Error::EINVAL as i32;


fn main() -> ExitCode {
//! Accepts prime candidate as a CLI parameter, checks a store of primes,
//! adding to the store as it works on the candidate.
    let context: Context = match env::args().nth(1) {
        Some(arg) => Context::new(&arg).unwrap_or_else(|err: String| {
            println!("\nError: {err}");
            process::exit(EINVAL);
        }),
        None => {
            println!("Usage: primer <number>\n");
            process::exit(EINVAL);
        }
    };

    let n = context.n;
    println!("U64 store: {}", context.store.int);
    println!("BIG store: {}", context.store.big);
    println!("{} PRIME? {}", n, is_prime(n));
    ExitCode::SUCCESS
}


fn is_prime(n: u64) -> bool {
    let bound = n.isqrt();
    bound > n // FIXME!
}
