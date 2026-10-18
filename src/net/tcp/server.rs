//! server.rs
//!

use std::{result::Result, error::Error};
use std::io::{self, BufRead, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::sync::{mpsc, Arc};
use std::borrow::Cow;
use std::time::Duration;

/// handler_echo
pub fn handler_echo(dbg: bool, ec: Arc<[u8]>, stream: TcpStream) ->
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
    if *ec == *r.as_bytes() { return Err(r.into()); }
  }
  Ok(())
}

/// thread_server
pub fn thread_server(dbg: bool, ec: Cow<'static, [u8]>,
  handler: fn (bool, Arc<[u8]>, TcpStream) -> Result<(), Box<dyn Error>>,
  host: Cow<'static, str>, port: u16) ->
  thread::JoinHandle<io::Result<()>> {
  thread::spawn(move || -> io::Result<()> {
    let hp = format!("{}:{}", host, port);
    let listener = TcpListener::bind(&hp)?;
    listener.set_nonblocking(true)?;
    if dbg { println!("Server listening on {}", hp); }
    let (tx, rx) = mpsc::channel::<Arc<[u8]>>(); // mpsc::sync_channel::<_>(2);
    loop {
      match rx.try_recv() { // before create thread
      Ok(b) => { if *ec == *b { break; } }
      Err(mpsc::TryRecvError::Empty) => {} // do nothing
      Err(mpsc::TryRecvError::Disconnected) => { break; } // dropped all ctx
      }
      match listener.accept() {
      Ok((stream, addr)) => {
        if dbg { println!("New connection: {}", addr); } // stream.peer_addr()?
        stream.set_nonblocking(false)?; // ignore listener blocking mode
        let ctx = tx.clone(); // clone to move (ctx will be dropped by closure)
        let cec: Arc<[u8]> = ec.clone().into();
        thread::spawn(move || {
          let b = Arc::clone(&cec);
          if let Err(e) = handler(dbg, cec, stream) {
            if *b == *e.to_string().as_bytes() { ctx.send(b).expect("send"); }
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
