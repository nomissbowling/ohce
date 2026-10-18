//! client.rs
//!

use std::io::{self, BufRead, Write};
use std::net::{ToSocketAddrs, TcpStream};
use std::thread;
use std::sync::{mpsc, Arc};
use std::borrow::Cow;
use std::time::Duration;

/// reader
pub fn reader(dbg: bool, ec: Arc<[u8]>, stream: TcpStream,
  tx: mpsc::Sender<Arc<[u8]>>) ->
  io::Result<()> {
  let mut br = io::BufReader::new(stream);
  let mut s = String::new();
  while br.read_line(&mut s)? > 0 {
    let r = format!("{}", s.trim_end());
    if dbg { println!("Client Received: {}", r); }
    tx.send(r.as_bytes().into()).expect("send");
    s.clear();
    if *ec == *r.as_bytes() { break; }
  }
  Ok(())
}

/// thread_client
pub fn thread_client(dbg: bool, ec: Cow<'static, [u8]>,
  handler_reader: fn (bool, Arc<[u8]>, TcpStream,
    tx: mpsc::Sender<Arc<[u8]>>) -> io::Result<()>,
  host: Cow<'static, str>, port: u16,
  irx: mpsc::Receiver<Arc<str>>, otx: mpsc::Sender<Arc<[u8]>>) ->
  thread::JoinHandle<io::Result<()>> {
  thread::spawn(move || -> io::Result<()> {
    let hp = format!("{}:{}", host, port);
    let mut addrs = hp.to_socket_addrs()?;
    if let Some(addr) = addrs.find(|x| (*x).is_ipv4()) {
      let to = Duration::from_secs(1);
      match TcpStream::connect_timeout(&addr, to) { // connect(addr)
      Ok(stream) => {
        if dbg { println!("Connection Ok: {}", stream.peer_addr()?); }
        let read_stream = stream.try_clone()?; // clone for read to be moved
        let cec: Arc<[u8]> = ec.clone().into();
        let ctx = otx.clone(); // clone to move
        let mut handles = vec![];
        handles.push(thread::spawn(move || {
          if let Err(e) = handler_reader(dbg, cec, read_stream, ctx) {
            if dbg { eprintln!("Error handling BufReader: {}", e); }
          }
        }));
        let mut bw = io::BufWriter::new(&stream);
        for r in irx { bw.write_all(format!("{}\n", r).as_bytes())?; } // block
        bw.flush()?;
        for h in handles {
          let _ = h.join().expect("join");
        }
        drop(otx);
      }
      Err(e) => { if dbg { eprintln!("Connection NG: {}", e); } }
      }
    } else {
      if dbg { eprintln!("Invalid Host:Port Number"); }
    }
    Ok(())
  })
}
