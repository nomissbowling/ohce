//! ext.rs
//!

use crate::net::*;

use std::{result::Result, error::Error};
use std::thread;
use std::time::Duration;

/// server_client_test
pub fn server_client_test(host: &str, port: u16, cec: &str, sec: &str) ->
  Result<(), Box<dyn Error>> {
  let dbg = true;
  // let ec = b".".into(); // from static
  // let ec = ".".as_bytes().into(); // from static
  // let ec = EndCode{c: Cow::Borrowed(".".as_bytes())}; // from static
  // let ec = EndCode{c: Cow::Owned(format!("{}", ".").as_bytes())};
  // let ec = format!("{}", ".").as_bytes().to_vec().into();
  let ec = sec.as_bytes().to_vec().into();
  let shost = format!("{}", host).into();
  let mut s = String::new();
  s += "How are you ?\n";
  s += "second line\n";
  s += "third line\n";
  s += cec; // end code to terminate client
  s += "\n";
  s += sec; // end code to terminate server
  s += "\n";
  let dat = s.as_bytes().to_vec().into();
  let cec = cec.as_bytes().to_vec().into();
  let chost = format!("{}", host).into();
  let mut handles = vec![];
  handles.push(tcp::thread_server(dbg, ec, tcp::handler_echo, shost, port));
  thread::sleep(Duration::from_millis(50));
  handles.push(tcp::thread_client(dbg, cec, tcp::reader, chost, port, dat));
  for h in handles {
    let _ = h.join().expect("join");
  }
  Ok(())
}
