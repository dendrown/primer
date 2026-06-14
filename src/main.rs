/// primer:
/// A persisent prime producer for play!
///
/// Copyright (c) 2026 Dennis Drown
use std::env;
use std::process;
use anyhow::Result;
use nix::Error;

use primer::Context;

const EINVAL: i32 = Error::EINVAL as i32;


#[tokio::main]
async fn main() -> Result<()> {
//! Accepts prime candidate as a CLI parameter, checks a store of primes,
//! adding to the store as it works on the candidate.
    let context: Context = match env::args().nth(1) {
        Some(arg) => Context::new(&arg)?,
        None => {
            eprintln!("Usage: primer <number>\n");
            process::exit(EINVAL);
        }
    };

    let is_prime = is_prime(&context).await?;

    println!("\nIS {} PRIME? {}", context.n, is_prime);
    Ok(())
}


async fn is_prime(context: &Context) -> Result<bool> {
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

    async fn check_sub_prime(i: u64, primes: &Vec<u64>, context: &Context) -> Result<()> {
        let bound = i.isqrt();
        print!("Sub-prime? {i}...");
        if !is_multiple(i, bound, &primes) {
            println!("Y");
            context.store.add(i)?;
        } else {
            println!("N");
        };
        Ok(())
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
            check_sub_prime(factor, &primes, context).await?;
        }
        factor += 2
    }
    Ok(true)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_primes() {

        let primes: Vec<u64> = vec![2, 3, 5, 7];
        check_prime(true, &primes).await;
    }

    #[tokio::test]
    async fn test_non_primes() {

        let non_primes: Vec<u64> = vec![4, 9, 55, 21];
        check_prime(false, &non_primes).await;
    }

    async fn check_prime(yn: bool, candidates: &Vec<u64>) {
        for n in candidates.iter() {
            let context = Context::new(n.to_string().as_str()).unwrap();
            assert_eq!(yn, is_prime(&context).await.unwrap());
        }
    }
}
