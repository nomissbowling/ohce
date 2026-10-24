//! ext.rs
//!

use crate::{net::*, core::esc};

use std::{result::Result, error::Error};
use std::io::{self, BufRead};
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
    "", // test skip line
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

/// read_esc_bytes
pub fn read_esc_bytes<R>(dbg: bool, r: &mut Vec<Vec<u8>>, src: &mut R) ->
  io::Result<()> where R: BufRead {
  let mut dst = Vec::new();
  let bytes = esc::read_line_including_esc_sequence(&mut dst, src)?;
  if dbg {
    println!("read: {} bytes", bytes);
    println!("data: {:?}", String::from_utf8_lossy(&dst));
  }
  // check carry over
  if bytes > 0 { r.push(dst); read_esc_bytes(dbg, r, src)?; } // 1 when '\n'
  Ok(())
}

/// esc_sequence_test
pub fn esc_sequence_test(dat: &[u8], o: &str) -> Result<(), Box<dyn Error>> {
  // compare with both splitted
  let mut src = io::BufReader::new(&dat[..]);
  let mut dst = Vec::<Vec<u8>>::new();
  read_esc_bytes(true, &mut dst, &mut src)?;
  let r = dst.into_iter().map(|u|
    esc::trim_esc_sequence_u8s(u).expect("u8s")).collect::<Vec<_>>();
  assert_eq!(r, o.split_inclusive('\n').collect::<Vec<_>>());

  // repeat same test straightforward
  let mut src = io::BufReader::new(&dat[..]);
  let mut dst = Vec::<Vec<u8>>::new();
  read_esc_bytes(true, &mut dst, &mut src)?;
  let r = dst.into_iter().map(|u|
    esc::trim_esc_sequence_u8s(u).expect("u8s")).collect::<String>();
  assert_eq!(r, o);

  // all lines at once
  assert_eq!(esc::trim_esc_sequence_u8s(dat.to_vec())?, o);
  assert_eq!(esc::trim_esc_sequence(&String::from_utf8(dat.to_vec())?), o);
  Ok(())
}
