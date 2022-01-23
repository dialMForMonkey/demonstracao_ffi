
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

use std::os::unix::net::UnixStream;
use std::io::prelude::*;



fn main() {
    let mut stream = UnixStream::connect("/tmp/ipc.sock").unwrap();
    let (tx, rx) = channel();
    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_secs(1));

            match sys_info::hostname() {
                Ok(hostname) => {
                    tx.send(format!("Hostname is: {}", hostname)).unwrap();
                },
                Err(x) => {
                    tx.send( format!("error {:?}",x)).unwrap();
                }
            };
        }
    });
    loop {
        let _ = rx
            .try_recv()
            .map(|reply| stream.write_all(reply.as_bytes()));
    }
}