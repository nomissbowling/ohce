#![doc(html_root_url = "https://docs.rs/ohce/0.1.4")]
//! reverse echo
//!

// #![feature(gen_blocks)] // must use Nightly

pub mod core;

/// test with [-- --nocapture] or [-- --show-output]
#[cfg(test)]
mod tests {
  use std::{result::Result, error::Error};
  use super::core::*;
  use prayterm::{PrayTerm, Rgb};
  use crossterm::style::Color;

  /// test core
  #[test]
  fn test_core() -> Result<(), Box<dyn Error>> {
    let u8s = b"abc\n".to_vec();
    assert_eq!(format!("[{}]", trim_crlf_u8s(u8s)?), "[abc]");
    Ok(())
  }

  /// test prayterm
  #[test]
  fn test_prayterm() {
    let s = String::from_utf8("ABC".as_bytes().to_vec()).expect("u8s");
    let mut tm = PrayTerm::new(2).expect("construct");
    tm.begin().expect("begin");
    tm.wr(0, 48, 3, Color::Blue, Color::Yellow, &s).expect("wr");
    tm.wr(0, 49, 3, Rgb(240, 192, 32), Rgb(240, 32, 192), &s).expect("wr");
    tm.fin().expect("fin");
    assert_eq!(tm.w, 80);
    assert_eq!(tm.h, 50);
  }
}
