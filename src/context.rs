/// primer: context.rs
/// Everything needed to find prime numbers.
///
/// Copyright (c) 2026 Dennis Drown
use crate::Store;
use anyhow::{anyhow, Result};

#[derive(Debug, Default)]
pub struct Context {
    pub n: u64,
    pub store: Store,
}


impl Context {
   pub  fn new(arg: &str) -> Result<Context> {
        let n: u64 = match arg.parse() {
            Ok(n) if n > 1 => n,
            Ok(_) => {
                return Err(anyhow!("number must be two or higher"));
            }
            Err(e) => {
                return Err(anyhow!(e.to_string()));
            }
        };
        Ok(Context{n, store: Store::new()})
    }
}


