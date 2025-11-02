#[allow(unused_imports)]
use std::io::{Read, Write};
use std::{net::TcpStream, thread};

#[allow(dead_code)]
pub struct FileClient {
    address: String,
    port: u16,
    code: Option<u16>,
    connection: TcpStream,
    download_path: String,
}

impl FileClient {
    pub fn new(address: String, port: u16, code: Option<u16>, download_path: String) -> Self {
        let connection = TcpStream::connect(format!("{}:{}", address, port)).unwrap();

        FileClient {
            address,
            port,
            code,
            connection,
            download_path,
        }
    }

    pub fn run(&self) {
        let _stream = self.connection.try_clone().unwrap();
        let mut read_stream = self.connection.try_clone().unwrap();
        let mut write_stream = self.connection.try_clone().unwrap();
        // let mut read_stream = self.connection.try_clone().unwrap();

        // reading from the stream on threaded spawn
        thread::spawn(move || {
            let mut buf = [0u8; 1024];
            loop {
                match read_stream.read(&mut buf) {
                    Ok(0) => break,
                    Ok(n) => println!("Client recieved : {}", String::from_utf8_lossy(&buf[..n])),
                    Err(_) => println!("error occured"),
                }
            }
        });

        thread::spawn(move || {
            let stdin = std::io::stdin();
            loop {
                let mut input = String::new();
                if stdin.read_line(&mut input).is_err() {
                    break;
                }
                if write_stream.write_all(input.as_bytes()).is_err() {
                    break;
                }
                write_stream.flush().unwrap();
            }
        })
        .join()
        .ok();

        // writing to the stream on main thread
        // let stdin = std::io::stdin();
        // loop {
        //     let mut input = String::new();
        //     stdin.read_line(&mut input).unwrap();
        //     stream.write_all(input.as_bytes()).unwrap();
        //     stream.flush().unwrap();
        // }
    }
}
