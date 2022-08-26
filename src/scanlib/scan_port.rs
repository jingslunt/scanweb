use std::net;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
pub fn scan_port(host: &str, port: &u16) -> bool {
    let host = host.to_string();
    let port = *port;
    let (sender, receiver) = mpsc::channel();
    let t = thread::spawn(move || {
        match sender.send(net::TcpStream::connect((host.as_str(), port))) {
            Ok(()) => {} // everything good
            Err(_) => {} // we have been released, don't panic
        }
    });

    thread::sleep(Duration::new(3, 0));

    match receiver.try_recv() {
        Ok(Ok(handle)) => true, // we have a connection
        Ok(Err(_)) => false,    // connecting failed
        Err(mpsc::TryRecvError::Empty) => {
            drop(receiver);
            drop(t);
            // connecting took more than 3 seconds
            false
        }
        Err(mpsc::TryRecvError::Disconnected) => unreachable!(),
    }
}
