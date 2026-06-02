use std::env;
use std::process;
use nix::Error;
use std::process::ExitCode;

fn main() -> ExitCode {
    let arg = env::args().nth(1);
    let n: u64 = match arg {
        Some(s) => match s.parse() {
            Ok(n) if n > 1 => n,
            Ok(_) => {
                panic!("\nError: number must be two or higher");
            }
            Err(e) => {
                panic!("\nError: {e}");
            }
        },
        None => {
            println!("Usage: primer <number>\n");
            process::exit(Error::EINVAL as i32);
        }
    };

    println!("PRIME? {:?}", n);
    ExitCode::SUCCESS
}
