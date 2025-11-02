#[allow(unused_imports)]
use std::io::{Read, Write};
use std::{error::Error, net::TcpStream, path::Path};

use tracing::{error, info};

use crate::{download, shared::*};

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

    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        // let _stream = self.connection.try_clone().unwrap();
        // let mut read_stream = self.connection.try_clone().unwrap();
        // let mut write_stream = self.connection.try_clone().unwrap();
        // // let mut read_stream = self.connection.try_clone().unwrap();

        // // reading from the stream on threaded spawn
        // thread::spawn(move || {
        //     let mut buf = [0u8; 1024];
        //     loop {
        //         match read_stream.read(&mut buf) {
        //             Ok(0) => break,
        //             Ok(n) => println!("Client recieved : {}", String::from_utf8_lossy(&buf[..n])),
        //             Err(_) => println!("error occured"),
        //         }
        //     }
        // })
        // .join()
        // .ok();

        // thread::spawn(move || {
        //     let stdin = std::io::stdin();
        //     loop {
        //         let mut input = String::new();
        //         if stdin.read_line(&mut input).is_err() {
        //             break;
        //         }
        //         if write_stream.write_all(input.as_bytes()).is_err() {
        //             break;
        //         }
        //         write_stream.flush().unwrap();
        //     }
        // })
        // .join()
        // .ok();

        // thread::spawn(move || {});

        let mut stream = self.connection.try_clone().unwrap();

        info!("[+] client connected to the server");

        write_message(&mut stream, "download")?;

        let server_response_file = read_message(&mut stream).unwrap();

        if server_response_file == "error" {
            error!("Server responded with error check the file name or server logs");
            stream.shutdown(std::net::Shutdown::Both).unwrap();
            return Ok(());
        }

        let path = Path::new(&self.download_path);
        let file_name = path.file_name().unwrap().to_str().unwrap().to_string();

        let mut file = download::create_file(&file_name).unwrap();
        info!("Created file with name {}", file_name);
        info!("Starting to read file...");

        loop {
            let chunk = download::read_chunk(&mut stream).unwrap();
            match chunk {
                Some(chunk) => download::write_chunk_to_file(&mut file, &chunk),
                None => return Ok(()),
            }
        }
    }
}
