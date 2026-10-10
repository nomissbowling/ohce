#![doc(html_root_url = "https://docs.rs/ohce/0.0.1")]
//! reverse echo
//!

pub mod core;

/// test with [-- --nocapture] or [-- --show-output]
#[cfg(test)]
mod tests {
  use super::*;
  use prayterm::{PrayTerm, Rgb};
  use crossterm::style::Color;

  /// test a
  #[test]
  fn test_a() {
    assert_eq!(0, 0);
  }
}
