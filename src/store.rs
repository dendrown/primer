/// primer: store.rs
/// Persistent storage of Integer and Big Integer prime numbers
///
/// Copyright (c) 2026 Dennis Drown
use std::fs;
use std::io::{self, BufReader, BufWriter, Read, Write};
use std::mem::size_of;

const STORE_U64: &str = "primes-u64.dat";
const STORE_BIG: &str = "primes-big.dat";
const INT_BYTES: u64 = size_of::<u64>() as u64;


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
    pub fn new() -> Self {
    //! Ensures the store files exist, creating them if necessary
        let store = fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(STORE_U64).expect(STORE_U64);
        let meta = store.metadata().expect("Bad store");

        // Start out with [2,3] to facilitate +=2 for odd candidates
        if meta.len() < 2*INT_BYTES {
            let mut writer = BufWriter::new(store);
            for &prime in &[2u64, 3u64] {
                writer.write_all(&prime.to_ne_bytes()).expect("Cannot initialize store");
            }
        }

        println!("{}: {} primes", STORE_U64, meta.len()/INT_BYTES);
        Self::default()
    }


    pub fn add(&self, n: u64) -> Result<(), io::Error> {
    //! Adds a prime (verified by caller) to the store
    //! TODO: generalize for BigUint
        let store = fs::OpenOptions::new()
            .append(true)
            .open(STORE_U64)?;
        let mut writer = BufWriter::new(store);
        writer.write_all(&n.to_ne_bytes())?;
        Ok(())
    }

    pub fn read_all_int(&self) -> Result<Vec<u64>, io::Error> {
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
