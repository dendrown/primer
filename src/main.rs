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

    println!("\nIS {} PRIME? {}", context.n, is_prime(&context));
    ExitCode::SUCCESS
}


fn is_prime(context: &Context) -> bool {
    let bound = context.n.isqrt();

    let primes: Vec<u64> = context.store.read_all_int().expect("Store failure");

    // FIXME: Have the store find its own limits
    let factor_base: u64 = match primes.last() {
        Some(p) => *p,
        None => 0
    };

    // TODO: don't go past where we need from the store
    for prime in primes {
        if prime > bound {
            break;
        }
        if context.n.is_multiple_of(prime) {
            return false;
        }
        println!("Checking known {prime}");
    };

    let mut factor = factor_base + 2;
    while factor <= bound {
        println!("Checking {factor}");
        if context.n.is_multiple_of(factor) {
            return false;
        } else {
            // TODO: async prime check & store
            println!("..sub-prime? {factor}");
        }

        factor += 2
    }
    true
}

