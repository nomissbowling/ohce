//! client.rs
//!

use crate::net::*;

use std::io::{self, BufRead, Write};
use std::net::{ToSocketAddrs, TcpStream};
use std::thread;
use std::borrow::Cow;

/// reader
pub fn reader(dbg: bool, ec: EndCode, stream: TcpStream) ->
  io::Result<()> {
  let mut br = io::BufReader::new(stream);
  let mut s = String::new();
  while br.read_line(&mut s)? > 0 {
    let r = format!("{}", s.trim_end());
    if dbg { println!("Client Received: {}", r); }
    s.clear();
    if ec.c == r.as_bytes() { break; }
  }
  Ok(())
}

/// thread_client
pub fn thread_client(dbg: bool, ec: EndCode,
  handler_reader: fn (bool, EndCode, TcpStream) -> io::Result<()>,
  host: Cow<'static, str>, port: u16, dat: Cow<'static, [u8]>) ->
  thread::JoinHandle<io::Result<()>> {
  thread::spawn(move || -> io::Result<()> {
    let hp = format!("{}:{}", host, port);
    let mut addrs = hp.to_socket_addrs()?;
    if let Some(addr) = addrs.find(|x| (*x).is_ipv4()) {
      match TcpStream::connect(addr) {
      Ok(stream) => {
        if dbg { println!("Connection Ok: {}", stream.peer_addr()?); }
        let read_stream = stream.try_clone()?; // clone for read to be moved
        let cec = ec.dup(); // as clone and to move
        let mut handles = vec![];
        handles.push(thread::spawn(move || {
          if let Err(e) = handler_reader(dbg, cec.into(), read_stream) {
            if dbg { eprintln!("Error handling BufReader: {}", e); }
          }
        }));
        let mut bw = io::BufWriter::new(&stream);
        bw.write_all(&dat)?;
        bw.flush()?;
        for h in handles {
          let _ = h.join().expect("join");
        }
      }
      Err(e) => { if dbg { eprintln!("Connection NG: {}", e); } }
      }
    } else {
      if dbg { eprintln!("Invalid Host:Port Number"); }
    }
    Ok(())
  })
}
