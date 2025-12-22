use std::sync::mpmc;
use std::thread;

enum Event {
    Stop,
}

struct SystemObserver(mpmc::Sender<Event>);

struct WayQuickExecutor(mpmc::Receiver<Event>);

pub fn exec() {
    let (tx, rx) = mpmc::channel::<Event>();
    let handles = [
        thread::spawn(move || {
            SystemObserver::new(tx).run();
        }),
        thread::spawn(move || {
            WayQuickExecutor::new(rx).run();
        }),
    ];
    for handle in handles {
        handle.join();
    }
}

impl SystemObserver {
    fn new(sender: mpmc::Sender<Event>) -> Self {
        Self(sender)
    }

    fn run(&self) {
        use std::fs;
        use std::io::{self, Read};
        use std::os::unix::net::UnixListener;
        loop {
            let socket_path = "/tmp/WayQuick.sock";

            let _ = fs::remove_file(socket_path);

            let listener = UnixListener::bind(socket_path)?;

            println!("Server waiting for connection...");

            for stream in listener.incoming() {
                match stream {
                    Ok(mut stream) => {
                        println!("New client connected!");

                        let mut buffer = [0u8; 1024];
                        match stream.read(&mut buffer) {
                            Ok(n) if n > 0 => {
                                let message = String::from_utf8_lossy(&buffer[..n]);
                                println!("Received message: {}", message);
                            }
                            _ => eprintln!("Failed to read message."),
                        }
                    }
                    Err(e) => eprintln!("Error: {}", e),
                }
            }
        }
    }
}

impl WayQuickExecutor {
    fn new(receiver: mpmc::Receiver<Event>) -> Self {
        Self(receiver)
    }

    fn run(&self) {}
}
