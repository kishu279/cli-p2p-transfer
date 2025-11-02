#[allow(unused_imports)]
use std::{
    io::{BufRead, BufReader, Read, Write},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

#[allow(dead_code)]
pub struct FileServer {
    address: String,
    port: u16,
    max_connection: Option<u16>,
    code: Option<u16>,
    listener: TcpListener,
    // file_queue: file_....
    // connections: Arc...
}

impl FileServer {
    pub fn new(address: String, port: u16, max_connection: Option<u16>, code: Option<u16>) -> Self {
        // start the listener
        let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

        println!("Server started on port {}", port);

        FileServer {
            address,
            port,
            max_connection,
            code,
            listener, // instance of tcp listening,
        }
    }

    pub fn run(&self) {
        println!("Server listening on port {}", self.port);
        // listen for incoming request
        for stream in self.listener.incoming() {
            let stream = stream.unwrap();

            println!("New Connection : {:?}", stream.peer_addr().unwrap());

            thread::spawn(|| {
                Self::handle_connection(stream);
            });
        }
    }

    // handle connection to the server
    fn handle_connection(mut stream: TcpStream) {
        // let buf_reader = BufReader::new(&stream);
        // let http_request: Vec<_> = buf_reader
        //     .lines()
        //     .map(|result| result.unwrap())
        //     .take_while(|line| !line.is_empty())
        //     .collect();

        // println!(
        //     "https request from {:?} : {:?}",
        //     stream.peer_addr().unwrap(),
        //     http_request
        // );

        // single threaded test
        // thread::sleep(Duration::from_secs(5));

        let mut buf = [0u8; 1024];

        loop {
            let n = match stream.read(&mut buf) {
                Ok(0) => break,
                Ok(n) => n,
                Err(_) => 0,
            };

            println!("Server recieved {}", String::from_utf8_lossy(&buf[..n]));
            if stream
                .write_all(b"connected to server successfully\n")
                .is_err()
            {
                break;
            }
            stream.flush().ok();
        }
    }
}
