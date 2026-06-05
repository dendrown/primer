/// primer: store.rs
/// Persistent storage of Integer and Big Integer prime numbers
///
/// Copyright (c) 2026 Dennis Drown
//use std::fs;

const STORE_U64: &str = "primes-u64.dat";
const STORE_BIG: &str = "primes-big.dat";


#[derive(Debug)]
pub struct Store {
    pub int: &'static str,      // Filename for Integer primes
    pub big: &'static str,      // Filename for Big Integer primes
}


impl Default for Store {

    fn default() -> Self {
        Store {int: STORE_U64, big: STORE_BIG}
    }
}
