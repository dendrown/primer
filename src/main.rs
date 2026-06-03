/// primer:
/// A persisent prime producer for play!
///
/// Copyright (c) 2026 Dennis Drown
use std::env;
//use std::fs;
use std::process;
use std::process::ExitCode;
use nix::Error;

const EINVAL: i32 = Error::EINVAL as i32;
const STORE_U64: &str = "primes-u64.dat";
const STORE_BIG: &str = "primes-big.dat";


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


#[derive(Default)]
struct Context {
    n: u64,
    store: Store,
}


impl Context {

    fn new(arg: &String) -> Result<Context, String> {
        let n: u64 = match arg.parse() {
            Ok(n) if n > 1 => n,
            Ok(_) => {
                return Err("number must be two or higher".to_string());
            }
            Err(e) => {
                return Err(e.to_string());
            }
        };
        Ok(Context{n: n, store: Store::default()})
    }
}


struct Store {
    int: &'static str,

    big: &'static str
}

impl Default for Store {

    fn default() -> Self {
        Store {int: STORE_U64, big: STORE_BIG}
    }
}


fn is_prime(n: u64) -> bool {
    let bound = n.isqrt();
    bound > n // FIXME!
}
