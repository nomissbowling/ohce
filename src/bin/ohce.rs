//! ohce.rs
//!
//! # Build (bins)
//!
//! - cargo build --bins --release
//! - cargo build --bin ohce --release
//! - cat resource/sample.txt | target/release/ohce
//! - cat resource/sample.txt | target/release/ohce -o resource/sample_out.txt
//!
//! # Options
//!
//! - --so-col=&lt;RRGGBB (hex color)&gt; : RGB color for stdout
//! - --se-col=&lt;RRGGBB (hex color)&gt; : RGB color for stderr
//! - -c --color : with color
//! - -e --error : with stderr
//! - -o --output=FILE : output to file
//! - -a --append : append mode (use with --output)
//! - -p --port=PORT : with TCP port
//! - -t --terminator=STR : close listener when received it (use with --port)
//!

// #![feature(gen_blocks)] // must use Nightly

use ohce::{core::*, net::*};

use std::fs::OpenOptions;
use std::path::PathBuf;
use bpaf::{self, Bpaf, Parser};
use crossterm::{QueueableCommand, style::{self, Stylize, Color}}; // ResetColor

use std::{result::Result, error::Error};
use std::io::{self, Write};

/// Opts
#[derive(Debug, Clone, Bpaf)]
#[bpaf(options, version)]
pub struct Opts {
  /// nocapture (only for test)
  #[bpaf(long)]
  nocapture: bool,
  /// show_output (only for test)
  #[bpaf(long)]
  show_output: bool,
  /// RGB color for stdout
  #[bpaf(long, argument("RRGGBB (hex color)"), fallback("20C0F0".into()),
    guard(check_hex_col, "can not parse --so-col"))] // aqua
  so_col: String,
  /// RGB color for stderr
  #[bpaf(long, argument("RRGGBB (hex color)"), fallback("F0C020".into()),
    guard(check_hex_col, "can not parse --se-col"))] // orange
  se_col: String,
  /// with color
  #[bpaf(short, long)]
  color: bool,
  /// with error
  #[bpaf(short, long)]
  error: bool,
  /// output to file
  #[bpaf(short, long, argument("FILE"), fallback("".into()),
    guard(check_output, "--output does not exist"))]
  output: PathBuf,
  /// append (use with --output)
  #[bpaf(short, long)]
  append: bool,
  /// with TCP port
  #[bpaf(short, long, argument("PORT"), fallback(0),
    guard(check_port, "--port=PORT can not set listener"))]
  port: u16,
  /// close listener when received it (use with --port)
  #[bpaf(short, long, argument("STR"), fallback(".".into()))] // winsplit shlex
  terminator: String,
  /// verbose (use with --debug)
  #[bpaf(external(verbose))]
  verbose: usize,
  /// activate debug
  #[bpaf(short, long)]
  debug: bool
}

/// external verbose
pub fn verbose() -> impl Parser<usize> {
  bpaf::short('v').long("verbose").help("verbose -v or -vv (use with --debug)")
    .req_flag(()).many().map(|xs| xs.len())
    .guard(|&x| x <= 2, "verbose <= 2")
}

/// guard check_port
pub fn check_port(_p: &u16) -> bool {
  true // now always true, check listener "localhost:{*p}"
}

/// guard check_output
pub fn check_output(p: &PathBuf) -> bool {
  let n = p.display().to_string();
  if n == "" {
    true
  } else {
    open_output(p, true).is_some() // always append
  }
}

/// guard check_hex_col
pub fn check_hex_col(s: &String) -> bool {
  parse_hex_col(s).is_ok()
}

/// parse_hex_col
pub fn parse_hex_col(s: &String) -> Result<Color, Box<dyn Error>> {
  if s.len() == 6 {
    let v = (0..s.len()).step_by(2).map(|i|
      u8::from_str_radix(&s[i..i+2], 16)).collect::<Result<Vec<u8>, _>>()?;
    if v.len() == 3 {
      return Ok(Color::Rgb{r: v[0], g: v[1], b: v[2]});
    }
  }
  Err("RRGGBB".into())
}

/// ohce_proc
pub fn ohce_proc(op: &Opts, f: Feed, lu_rev: fn (String) -> String) ->
  Result<String, Box<dyn Error>> {
  let cols = [&op.so_col, &op.se_col].into_iter().map(|s|
    parse_hex_col(s)).collect::<Result<Vec<_>, _>>()?;
  let r = reverse(f.s);
  let mut s = String::new();
  s += &r;
  if op.color {
    io::stdout().queue(style::PrintStyledContent(r.as_str().with(cols[0])))?;
    // io::stdout().queue(ResetColor)?; // needless when PrintStyledContent
  } else {
    print!("{}", r);
  }
  if f.lf { s += "\n"; print!("{}", '\n'); }
  io::stdout().flush()?;
  if op.error {
    let e = lu_rev(r);
    s += &e;
    if op.color {
      io::stderr().queue(style::PrintStyledContent(e.with(cols[1])))?;
      // io::stderr().queue(ResetColor)?; // needless when PrintStyledContent
    } else {
      eprint!("{}", e);
    }
    if f.lf { s += "\n"; eprint!("{}", '\n'); }
    io::stderr().flush()?;
  }
  if let Some(mut bw) = open_output(&op.output, true) { // always append
    bw.write(s.as_bytes())?;
    bw.flush()?;
  }
  Ok(s)
}

/// open_output
pub fn open_output(p: &PathBuf, a: bool) -> Option<io::BufWriter<impl Write>> {
  if p.display().to_string() != "" {
    let mut o = OpenOptions::new();
    if a {
      if let Ok(w) = o.create(true).append(a).open(p) {
        Some(io::BufWriter::new(w))
      } else {
        None
      }
    } else {
      if let Ok(w) = o.create(true).write(true).truncate(true).open(p) {
        Some(io::BufWriter::new(w))
      } else {
        None
      }
    }
  } else {
    None
  }
}

/// ohce_through
pub fn ohce_through(_op: &Opts, f: Feed) -> Result<String, Box<dyn Error>> {
  let s = format!("[{}{}]\n", f.s, lf(f.lf));
  print!("{}", s);
  Ok(s)
}

/// main
pub fn main() -> Result<(), Box<dyn Error>> {
  let op = opts().run(); // touch output file by guard check_output when set it
  if op.debug {
    println!("{:?}", op);
    if op.verbose < 1 { return Ok(()); }
    println!("--nocapture: {}", op.nocapture); // (only for test)
    println!("--show-output: {}", op.show_output); // (only for test)
  }
  let mut handles = vec![];
  if op.port > 0 {
    let dbg = op.debug && op.verbose > 1;
    let ec = op.terminator.as_bytes().to_vec().into();
    let host = "127.0.0.1".into();
    let port = op.port;
    handles.push(tcp::thread_server(dbg, ec, tcp::handler_echo, host, port));
  }
  if let Some(bw) = open_output(&op.output, op.append) { drop(bw); } // release

  let lu_rev = lu_reverse; // lu_reverse_ascii
  for f in read_iter_a(io::stdin().lock()) { ohce_proc(&op, f, lu_rev)?; }
/* // must use Nightly
  let lt = "dummy lifetime";
  for f in read_iter(lt, io::stdin().lock()) { ohce_proc(&op, f, lu_rev)?; }
*/
/*
  let mut v: Vec<Feed> = vec![];
  read_feeds(&mut v, io::stdin().lock())?;
  for f in v { ohce_proc(&op, f, lu_rev)?; }
*/

  if op.port > 0 {
    for h in handles {
      let _ = h.join().expect("join");
    }
  }
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
    let op = opts().run();
    assert_eq!(ohce_proc(&op, Feed::new("xYz", false), lu_reverse_ascii)?,
      "zYx");
    Ok(())
  }

  /// test ohce_proc
  #[test]
  fn test_ohce_proc() -> Result<(), Box<dyn Error>> {
    let op = opts().run();
    assert_eq!(ohce_proc(&op, Feed::new("ＡｂＣ", false), lu_reverse)?,
      "ＣｂＡ");
    Ok(())
  }

  /// test ohce_through
  #[test]
  fn test_ohce_through() -> Result<(), Box<dyn Error>> {
    let op = opts().run();
    assert_eq!(ohce_through(&op, Feed::new("through", false))?,
      "[through]\n");
    Ok(())
  }
}
