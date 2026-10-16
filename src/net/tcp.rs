//! tcp.rs
//!

use std::{result::Result, error::Error};
use std::io::{self, BufRead, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::sync::mpsc;
use std::time::Duration;

/// client_echo
pub fn client_echo(dbg: bool, ec: &'static [u8], stream: TcpStream) ->
  Result<(), Box<dyn Error>> {
  let mut br = io::BufReader::new(&stream); // (stream.try_clone()?);
  let mut bw = io::BufWriter::new(&stream); // (stream);
  let mut s = String::new();
  while br.read_line(&mut s)? > 0 {
    let r = format!("{}", s.trim_end());
    if dbg { println!("Received: {}", r); }
    bw.write_all(s.as_bytes())?;
    bw.flush()?;
    s.clear();
    if r.as_bytes() == ec { return Err(r.into()); }
  }
  Ok(())
}

/// handle_server
pub fn handle_server(dbg: bool, ec: &'static [u8],
  client: fn (bool, &'static [u8], TcpStream) -> Result<(), Box<dyn Error>>,
  host: &'static str, port: u16) ->
  thread::JoinHandle<io::Result<()>> {
  thread::spawn(move || -> io::Result<()> {
    let listener = TcpListener::bind(format!("{}:{}", host, port))?;
    listener.set_nonblocking(true)?;
    let (tx, rx) = mpsc::channel(); // mpsc::sync_channel::<Vec<u8>>(2);
    if dbg { println!("Server listening on {}:{}", host, port); }
    loop {
      match rx.try_recv() { // before create thread
      Ok(r) => { if r == ec { break; } }
      Err(mpsc::TryRecvError::Empty) => {} // do nothing
      Err(mpsc::TryRecvError::Disconnected) => { break; } // dropped all ctx
      }
      match listener.accept() {
      Ok((stream, addr)) => {
        if dbg { println!("New connection: {}", addr); } // stream.peer_addr()?
        stream.set_nonblocking(false)?; // ignore listener blocking mode
        let ctx = tx.clone(); // clone to move (ctx will be dropped by closure)
        thread::spawn(move || {
          if let Err(e) = client(dbg, ec, stream) {
            if e.to_string().as_bytes() == ec { ctx.send(ec).expect("send"); }
            else { if dbg { eprintln!("Error handling client: {}", e); } }
          }
          if dbg { println!("Disconnected."); }
        });
      }
      Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {} // do nothing
      Err(e) => { if dbg { eprintln!("Connection failed: {}", e); } }
      }
      thread::sleep(Duration::from_millis(50));
    }
    drop(tx);
    Ok(())
  })
}
