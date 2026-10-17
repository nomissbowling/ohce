//! tcp.rs
//!

use std::{result::Result, error::Error};
use std::io::{self, BufRead, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::sync::mpsc;
use std::borrow::Cow;
use std::time::Duration;

/// EndCode (static Cow for &amp;[u8] or Vec&lt;u8&gt;)
pub struct EndCode {
  /// c
  pub c: Cow<'static, [u8]>
}

/// EndCode
impl EndCode {
  /// dup
  pub fn dup(&self) -> Self { self.to_vec().into() }

  /// to_vec
  pub fn to_vec(&self) -> Vec<u8> { (*self.c).iter().map(|&c| c).collect() }
}

/// impl From for EndCode
impl<C> From<C> for EndCode where Cow<'static, [u8]>: From<C> {
  /// from
  fn from(c: C) -> Self { EndCode{c: c.into()} }
}

/// handler_echo
pub fn handler_echo(dbg: bool, ec: EndCode, stream: TcpStream) ->
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
    if ec.c == r.as_bytes() { return Err(r.into()); }
  }
  Ok(())
}

/// thread_server
pub fn thread_server(dbg: bool, ec: EndCode,
  handler: fn (bool, EndCode, TcpStream) -> Result<(), Box<dyn Error>>,
  host: Cow<'static, str>, port: u16) ->
  thread::JoinHandle<io::Result<()>> {
  thread::spawn(move || -> io::Result<()> {
    let hp = format!("{}:{}", host, port);
    let listener = TcpListener::bind(&hp)?;
    listener.set_nonblocking(true)?;
    if dbg { println!("Server listening on {}", hp); }
    let (tx, rx) = mpsc::channel(); // mpsc::sync_channel::<Vec<u8>>(2);
    loop {
      match rx.try_recv() { // before create thread
      Ok(b) => { if ec.c == b { break; } }
      Err(mpsc::TryRecvError::Empty) => {} // do nothing
      Err(mpsc::TryRecvError::Disconnected) => { break; } // dropped all ctx
      }
      match listener.accept() {
      Ok((stream, addr)) => {
        if dbg { println!("New connection: {}", addr); } // stream.peer_addr()?
        stream.set_nonblocking(false)?; // ignore listener blocking mode
        let ctx = tx.clone(); // clone to move (ctx will be dropped by closure)
        let cec = ec.dup(); // as clone and to move
        let b = ec.to_vec(); // like clone for reuse (not clone)
        thread::spawn(move || {
          if let Err(e) = handler(dbg, cec.into(), stream) {
            if b == e.to_string().as_bytes() { ctx.send(b).expect("send"); }
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
