//! ext.rs
//!

use crate::net::*;

use std::{result::Result, error::Error};
use std::thread;
use std::sync::{mpsc, Arc};
// use std::borrow::Cow;
use std::time::Duration;

/// server_client_test
pub fn server_client_test(host: &str, port: u16, cec: &str, sec: &str) ->
  Result<(), Box<dyn Error>> {
  let dbg = true;
  // let ec = b".".into(); // from static
  // let ec = ".".as_bytes().into(); // from static
  // let ec = Cow::Borrowed(".".as_bytes()); // from static
  // let ec = Cow::Owned(format!("{}", ".").as_bytes().to_vec());
  // let ec = format!("{}", ".").as_bytes().to_vec().into();
  let ec = sec.as_bytes().to_vec().into();
  let shost = format!("{}", host).into();
  let v = vec![
    "How are you ?",
    "second line",
    "third line",
    cec, // end code to terminate client
    sec]; // end code to terminate server
  let cec = cec.as_bytes().to_vec().into();
  let chost = format!("{}", host).into();
  let (tx, rx) = mpsc::channel::<Arc<str>>(); // mpsc::sync_channel::<_>(2);
  let (xt, xr) = mpsc::channel::<Arc<[u8]>>(); // mpsc::sync_channel::<_>(2);
  let mut handles = vec![];
  handles.push(tcp::thread_server(dbg, ec, tcp::handler_echo, shost, port));
  thread::sleep(Duration::from_millis(50));
  handles.push(tcp::thread_client(dbg, cec, tcp::reader, chost, port, rx, xt));
  for t in &v { tx.send((*t).into())?; }
  drop(tx); // need for close rx in client thread (for r in irx {}) un-block
  for h in handles {
    let _ = h.join().expect("join");
  }
  let o = xr.into_iter().map(|r|
    String::from_utf8(r.to_vec()).expect("utf8")).collect::<Vec<_>>();
  if dbg { println!("[{:?}]", o); }
  assert_eq!(o, &v[..v.len()-1]);
  Ok(())
}
