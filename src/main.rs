/// primer:
/// A persisent prime producer for play!
///
/// Copyright (c) 2026 Dennis Drown
use std::env;
use std::process;
use std::process::ExitCode;
use anyhow;
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

    let is_prime = is_prime(&context).expect("Internal error");

    println!("\nIS {} PRIME? {}", context.n, is_prime);
    ExitCode::SUCCESS
}


fn is_prime(context: &Context) -> Result<bool, anyhow::Error> {
//! Determines if the given number is prime, sync'ing intermediate primes with the store.

    fn is_multiple(n: u64, bound: u64, primes: &Vec<u64>) -> bool {
        for prime in primes {
            if *prime > bound {
                break;
            }

            if n.is_multiple_of(*prime) {
                return true;
            }
        };
        false
    }

    let bound = context.n.isqrt();
    let primes: Vec<u64> = context.store.read_all_int()?; // TODO: just pull to bound from the store

    println!("Checks are bounded at {bound}");

    if is_multiple(context.n, bound, &primes) {
        return Ok(false);
    }

    // FIXME: Have the store find its own limits
    let factor_base: u64 = match primes.last() {
        Some(p) => *p,
        None => 0
    };
    let mut factor = factor_base + 2;
    while factor <= bound {
        if context.n.is_multiple_of(factor) {
            return Ok(false);
        } else {
            // TODO: async sub-prime check
            print!("Sub-prime? {factor}...");
            if !is_multiple(factor, bound, &primes) {
                println!("Y");
                context.store.add(factor)?;
            } else {
                println!("N");
            }
        }
        factor += 2
    }
    Ok(true)
}

