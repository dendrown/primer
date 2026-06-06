/// primer: context.rs
/// Everything needed to find prime numbers.
///
/// Copyright (c) 2026 Dennis Drown
use crate::Store;

#[derive(Debug, Default)]
pub struct Context {
    pub n: u64,
    pub store: Store,
}


impl Context {
   pub  fn new(arg: &str) -> Result<Context, String> {
        let n: u64 = match arg.parse() {
            Ok(n) if n > 1 => n,
            Ok(_) => {
                return Err("number must be two or higher".to_string());
            }
            Err(e) => {
                return Err(e.to_string());
            }
        };
        Ok(Context{n, store: Store::new()})
    }
}
