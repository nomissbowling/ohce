//! core.rs
//!

// in lib.rs #![feature(gen_blocks)] // must use Nightly
use genawaiter::rc::{Gen as Gen_, Co as Co_};

use std::{result::Result, error::Error};
use std::io::{self, BufRead};

/// Feed
pub struct Feed {
  /// String
  pub s: String,
  /// line feed
  pub lf: bool
}

/// Feed
impl Feed {
  /// new
  pub fn new(s: &str, lf: bool) -> Self {
    Feed{s: s.to_string(), lf}
  }
}

/// reverse
pub fn reverse(s: String) -> String {
  s.chars().rev().collect::<String>()
}

/// lu_reverse_ascii
pub fn lu_reverse_ascii(s: String) -> String {
  s.chars().into_iter().map(|c| {
    if c.is_ascii_alphabetic() {
      if c.is_ascii_lowercase() {
        c.to_ascii_uppercase()
      } else if c.is_ascii_uppercase() {
        c.to_ascii_lowercase()
      } else {
        c
      }
    } else {
      c
    }
  }).collect::<String>()
}

/// lu_reverse
pub fn lu_reverse(s: String) -> String {
  s.chars().into_iter().map(|c| {
    if c.is_alphabetic() {
      if c.is_lowercase() {
        c.to_uppercase().collect()
      } else if c.is_uppercase() {
        c.to_lowercase().collect()
      } else {
        vec![c]
      }
    } else {
      vec![c]
    }
  }).flatten().collect::<String>()
}

/// lf
pub fn lf(b: bool) -> String {
  if b { "\n" } else { "" } .to_string()
}

/// trim_crlf
pub fn trim_crlf(s: &String) -> String {
  s.trim_end_matches(['\r', '\n']).to_string()
}

/// trim_crlf_u8s
pub fn trim_crlf_u8s(u8s: Vec<u8>) -> Result<String, Box<dyn Error>> {
  Ok(trim_crlf(&String::from_utf8(u8s)?))
}

/// read_iter_a
pub fn read_iter_a<R>(src: R) -> impl IntoIterator<Item=Feed>
  where R: BufRead {
// (src: R) better OK
  let mut it = src.lines().peekable();
  Gen_::<Feed, (), _>::new(|co: Co_<Feed>| async move {
    loop {
      let s = if let Some(l) = it.next() { l.expect("") } else { break };
      let lf = !it.peek().is_none();
      co.yield_(Feed{s, lf}).await;
      if !lf { break };
    }
  })
}

/* // must use Nightly
/// read_iter
pub fn read_iter<'a, R>(_lt: &'a str, src: R) -> impl Iterator<Item=Feed> + 'a
  where R: BufRead + 'a {
// (src: R) better OK
  let mut it = src.lines().peekable();
  gen move {
    loop {
      let s = if let Some(l) = it.next() { l.expect("") } else { break };
      let lf = !it.peek().is_none();
      yield Feed{s, lf};
      if !lf { break };
    }
  }
}
*/

/// read_feeds
pub fn read_feeds<R>(dst: &mut Vec<Feed>, src: R) -> io::Result<()>
  where R: BufRead {
// (src: R) better OK
  let mut it = src.lines().peekable();
  loop {
    // let s = it.next().map_or(Ok(format!("")), |l| -> io::Result<String> { Ok(l?) })?;
    // let s = it.next().map_or(Ok(format!("")), |l| -> io::Result<String> { l })?;
    // let s = it.next().map_or(Ok(format!("")), |l| l)?;
    // let s = it.next().map_or(format!(""), |l| l.expect(""));
    // let s = match it.next() { Some(l) => l?, None => format!("") };
    // let s = if let Some(l) = it.next() { l.expect("") } else { break };
    let s = if let Some(l) = it.next() { l? } else { break };
    let lf = !it.peek().is_none();
    dst.push(Feed{s, lf});
    if !lf { break };
  }
/* // (mut src: R) good OK s contains \n
  let mut s = String::new();
  loop {
    src.read_line(&mut s)?;
    let lf = !src.fill_buf()?.is_empty();
    dst.push(Feed{s: trim_crlf(&s), lf});
    s.clear();
    if !lf { break };
  }
*/
/* // (src: R) always added \n
  for s in src.lines() {
    dst.push(Feed{s: s?, lf: true});
  }
*/
  Ok(())
}
