/// primer: store.rs
/// Persistent storage of Integer and Big Integer prime numbers
///
/// Copyright (c) 2026 Dennis Drown
use std::fs;
use std::io::{self, BufReader, Read};

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


impl Store {
    pub fn read_all_int(self) -> Result<Vec<u64>, io::Error> {
    //! Read all prime numbers currently in the Integer store
    //! TODO: Only read up to a requested value for n
        let /*mut*/ store = fs::OpenOptions::new()
            .read(true)
            //.append(true)
            .open(self.int)?;
        let mut reader = BufReader::new(store);
        let mut buffer = [0u8; 8];
        let mut primes = Vec::<u64>::new();

        loop {
            match reader.read_exact(&mut buffer) {
                Ok(()) => primes.push(u64::from_ne_bytes(buffer)),
                Err(ref e) if e.kind() == io::ErrorKind::UnexpectedEof => break,
                Err(e) => return Err(e),
            }
        }
        Ok(primes)
    }
}
