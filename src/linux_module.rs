use crate::quickapp::Event;
use std::sync::mpsc;
use std::thread;

struct SystemObserver(mpsc::Sender<Event>);

pub fn exec() {
    let (tx, rx) = mpsc::channel::<Event>();
    let handles = [
        thread::spawn(move || {
            SystemObserver::new(tx).run();
        }),
        thread::spawn(move || {
            crate::quickapp::app_run(rx);
        }),
    ];
    for handle in handles {
        handle.join();
    }
}

impl SystemObserver {
    fn new(sender: mpsc::Sender<Event>) -> Self {
        Self(sender)
    }

    fn run(&self) {
        use std::fs;
        use std::io::Read;
        use std::os::unix::net::UnixListener;
        loop {
            let socket_path = "/tmp/WayQuick.sock";

            let _ = fs::remove_file(socket_path);

            let Ok(listener) = UnixListener::bind(socket_path) else {
                eprintln!("Socket binding failed");
                return;
            };

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
