//! A tool to fatefully exit the process without panics
//! # (Pseudo-)Example:
//! ```ignore
//! use std::env;
//! use rand;
//! use fateful::fatal;
//!
//! fn main() {
//!     let args: Vec<String> = env::args().collect();
//!     if args.len() < 3 {
//!         fatal!("missing random items to choose of");
//!     }
//!
//!     let random_items: &Vec<String> = &args[1..args.len()].to_vec();
//!     let index = (rand::random::<f32>() * random_items.len() as f32).floor() as usize;
//!     println!("Random: {} 🎉", random_items[index]);
//! }
//! ```

#[macro_export]
/// Throws a fatal error to the standard error output
/// Then exits with an exit code 1
macro_rules! fatal {
  () => { ::std::process::exit(1) };
  ($($arg:tt)*) => {
    {
      ::std::eprintln!($($arg)*);
      $crate::fatal!()
     }
  };
}

#[macro_export]
/// Returns a prefix of Error:
macro_rules! err_prefix { () => {"Error: {}"} }

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unreachable_code, dead_code)]
    fn test_expansions_compiles() {
        fatal!();
        fatal!("Error");
        fatal!("Err{}", "or");
    }
}
