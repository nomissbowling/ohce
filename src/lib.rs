#![doc(html_root_url = "https://docs.rs/ohce/0.2.5")]
//! reverse echo
//!

// #![feature(gen_blocks)] // must use Nightly

pub mod core;
pub mod net;
pub mod ext;

/// test with [-- --nocapture] or [-- --show-output]
#[cfg(test)]
mod tests {
  use std::{result::Result, error::Error};
  use super::{core::*, ext::*};
  use prayterm::{PrayTerm, Rgb};
  use crossterm::style::Color;

  /// test core
  #[test]
  fn test_core() -> Result<(), Box<dyn Error>> {
    let u8s = b"abc\n".to_vec();
    assert_eq!(format!("[{}]", trim_crlf_u8s(u8s)?), "[abc]");
    Ok(())
  }

  /// test esc sequence
  #[test]
  fn test_esc_sequence() -> Result<(), Box<dyn Error>> {
    let dat = b"Hello \x1b[31;1mColor\n\x1b[0mNext Line\x1b[0m\n\nABC";
    let o = "Hello Color\nNext Line\n\nABC";
    assert_eq!(esc_sequence_test(dat, o)?, ());
    Ok(())
  }

  /// test server client
  #[test]
  fn test_server_client() -> Result<(), Box<dyn Error>> {
    assert_eq!(server_client_test("127.0.0.1", 23123, "end.", ".")?, ());
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
