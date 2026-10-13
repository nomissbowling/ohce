//! ohce.rs
//!
//! - cargo build --bins --release
//! - cargo build --bin ohce --release
//! - cat resource/sample.txt | target/release/ohce
//!

// #![feature(gen_blocks)] // must use Nightly

use ohce::core::*;

use crossterm::{QueueableCommand, style::{self, Stylize, Color}}; // ResetColor

use std::{result::Result, error::Error};
use std::io::{self, Write};

/// ohce_proc_ascii
pub fn ohce_proc_ascii(f: Feed) -> Result<(), Box<dyn Error>> {
  let aq = Color::Rgb{r: 32, g: 192, b: 240};
  let r = reverse(f.s);
  print!("{}{}", r, lf(f.lf));
  io::stdout().flush()?;
  // eprint!("{}{}", lu_reverse_ascii(r), lf(f.lf));
  io::stderr().queue(style::PrintStyledContent(lu_reverse_ascii(r).with(aq)))?;
  // io::stderr().queue(ResetColor)?; // needless when use PrintStyledContent
  if f.lf { eprint!("{}", '\n'); }
  io::stderr().flush()?;
  Ok(())
}

/// ohce_proc
pub fn ohce_proc(f: Feed) -> Result<(), Box<dyn Error>> {
  let orange = Color::Rgb{r: 240, g: 192, b: 32};
  let r = reverse(f.s);
  print!("{}{}", r, lf(f.lf));
  io::stdout().flush()?;
  // eprint!("{}{}", lu_reverse(r), lf(f.lf));
  io::stderr().queue(style::PrintStyledContent(lu_reverse(r).with(orange)))?;
  // io::stderr().queue(ResetColor)?; // needless when use PrintStyledContent
  if f.lf { eprint!("{}", '\n'); }
  io::stderr().flush()?;
  Ok(())
}

/// ohce_through
pub fn ohce_through(f: Feed) -> Result<(), Box<dyn Error>> {
  println!("[{}{}]", f.s, lf(f.lf));
  Ok(())
}

/// main
pub fn main() -> Result<(), Box<dyn Error>> {
  for f in read_iter_a(io::stdin().lock()) { ohce_proc(f)? }
/* // must use Nightly
  let lt = "dummy lifetime";
  for f in read_iter(lt, io::stdin().lock()) { ohce_proc(f)? }
*/
/*
  let mut v: Vec<Feed> = vec![];
  read_feeds(&mut v, io::stdin().lock())?;
  for f in v { ohce_proc(f)? }
*/
  Ok(())
}

/// test with [-- --nocapture] or [-- --show-output]
#[cfg(test)]
mod tests {
  use std::{result::Result, error::Error};
  use ohce::core::Feed; // lib
  use crate::*; // main

  /// test ohce_proc_ascii
  #[test]
  fn test_ohce_proc_ascii() -> Result<(), Box<dyn Error>> {
    assert_eq!(ohce_proc_ascii(Feed::new("xYz", false))?, ());
    Ok(())
  }

  /// test ohce_proc
  #[test]
  fn test_ohce_proc() -> Result<(), Box<dyn Error>> {
    assert_eq!(ohce_proc(Feed::new("ＡｂＣ", false))?, ());
    Ok(())
  }

  /// test ohce_through
  #[test]
  fn test_ohce_through() -> Result<(), Box<dyn Error>> {
    assert_eq!(ohce_through(Feed::new("through", false))?, ());
    Ok(())
  }
}
