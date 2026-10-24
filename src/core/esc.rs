//! esc.rs
//!
//! src.read_line() BufRead stops read when found LF
//!
//! - [github](https://gist.github.com/rust-play/f09a4ff76b8193546c41fef9b04f36ab)
//! - [play rust-lang](https://play.rust-lang.org/?version=stable&mode=release&edition=2024&gist=f09a4ff76b8193546c41fef9b04f36ab)
//!

use std::{result::Result, error::Error};
use std::io::{self, BufRead};

/// trim_esc_sequence
pub fn trim_esc_sequence(s: &str) -> String {
  trim_esc_sequence_bufreader(&mut s.as_bytes()).map_or(s.to_string(), |o| o)
}

/// trim_esc_sequence_u8s
pub fn trim_esc_sequence_u8s(u8s: Vec<u8>) -> Result<String, Box<dyn Error>> {
  trim_esc_sequence_bufreader(&mut io::BufReader::new(io::Cursor::new(u8s)))
}

/// trim_esc_sequence_bufreader
pub fn trim_esc_sequence_bufreader<R>(src: &mut R) ->
  Result<String, Box<dyn Error>> where R: BufRead {
  let mut dst = Vec::<u8>::new();
  let mut esc = Vec::<u8>::new(); // to catch trimmed ESC sequence but not use
  loop {
    let (available_bytes, should_break) = {
      let buf = src.fill_buf()?; // read non blocking
/*
      println!("[{}] [{:?}] [{}]",
        String::from_utf8(dst.to_vec())?,
        String::from_utf8(esc.to_vec())?,
        String::from_utf8(buf.to_vec())?); // [dst] [debug ESC bytes] [src]
*/
      if buf.is_empty() {
        (0, true) // EOF
      } else {
        if let Some(pos) = buf.iter().position(|&b| b == 0x1b) { // scan ESC
          if pos > 0 { // found ESC (not on the head)
            dst.extend_from_slice(&buf[..pos]);
            (pos, false) // parse ESC after continue, next pos will be always 0
          } else { // found ESC on the head
            let (consumed, _done) = parse_esc_sequence(&mut esc, &buf[pos..]);
            (pos + consumed, false) // always set to false (ignore done status)
          }
        } else { // not found ESC
          dst.extend_from_slice(buf);
          (buf.len(), false)
        }
      }
    };
    if available_bytes > 0 { src.consume(available_bytes); }
    if should_break { break; }
  }
  Ok(String::from_utf8(dst)?)
}

/// read_line_including_esc_sequence
pub fn read_line_including_esc_sequence<R>(dst: &mut Vec<u8>, src: &mut R) ->
  io::Result<usize> where R: BufRead {
  let mut total_bytes = 0;
  let mut lf_found = false;
  loop {
    let (available_bytes, should_break) = {
      let buf = src.fill_buf()?; // read non blocking
      if buf.is_empty() {
        (0, true) // EOF
      } else if !lf_found {
        if let Some(lf_pos) = buf.iter().position(|&b| b == b'\n') { // scan LF
          lf_found = true;
          let next_pos = lf_pos + 1;
          dst.extend_from_slice(&buf[..next_pos]);
          if next_pos < buf.len() && buf[next_pos] == 0x1b { // ESC after LF
            let (consumed, done) = parse_esc_sequence(dst, &buf[next_pos..]);
            (next_pos + consumed, done) // continue parse ESC sequence if false
          } else if next_pos == buf.len() {
            (next_pos, false) // check ESC next loop when LF on the end of buf
          } else {
            (next_pos, true) // no ESC after LF
          }
        } else { // not found LF
          dst.extend_from_slice(buf);
          (buf.len(), false)
        }
      } else { // parse ESC sequence when found LF
        if buf[0] == 0x1b { parse_esc_sequence(dst, buf) } else { (0, true) }
      }
    };
    if available_bytes > 0 {
      src.consume(available_bytes);
      total_bytes += available_bytes;
    }
    if should_break { break; }
  }
  Ok(total_bytes)
}

/// parse_esc_sequence (consumed, done) continue parse ESC sequence if false
pub fn parse_esc_sequence(dst: &mut Vec<u8>, buf: &[u8]) -> (usize, bool) {
  if buf.is_empty() || buf[0] != 0x1b { return (0, true); }
  if buf.len() > 1 && buf[1] == b'[' { // CSI expects `\x1b[`
    // scan terminator (0x40-0x7E) after 3rd byte
    for i in 2..buf.len() {
      let b = buf[i];
      if (0x40..=0x7E).contains(&b) { // found
        let end = i + 1;
        dst.extend_from_slice(&buf[..end]);
        return (end, true);
      }
    }
    dst.extend_from_slice(buf); // not found anything before the end of buf
    (buf.len(), false)
  } else { // single `\x1b` or 2 bytes (ex. `\x1bN`) not `\x1b[`
    let end = if buf.len() >= 2 { 2 } else { 1 };
    dst.extend_from_slice(&buf[..end]);
    (end, end == 2)
  }
}
