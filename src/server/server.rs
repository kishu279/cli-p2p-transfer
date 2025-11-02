#[allow(unused_imports)]
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use std::{error::Error, fs::File, path::Path};
#[allow(unused_imports)]
use std::{
    io::{BufRead, BufReader, Read, Write},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use tracing::error;

use crate::shared;

#[allow(dead_code)]
pub struct FileServer {
    address: String,
    port: u16,
    max_connection: Option<u16>,
    code: Option<u16>,
    listener: TcpListener,
    path: String,
    // sessions: Arc<Mutex<HashMap<String, TcpStream>>>,
}

impl FileServer {
    pub fn new(
        address: String,
        port: u16,
        max_connection: Option<u16>,
        code: Option<u16>,
        path: String,
    ) -> Self {
        // start the listener
        let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

        println!("Server started on port {}", port);

        FileServer {
            address,
            port,
            max_connection,
            code,
            listener, // instance of tcp listening,
            // sessions: Arc::new(Mutex::new(HashMap::new())),
            path,
        }
    }

    pub fn run(&self) {
        println!("Server listening on port {}", self.port);
        // listen for incoming request
        for stream in self.listener.incoming() {
            let stream = stream.unwrap();

            println!("New Connection : {:?}", stream.peer_addr().unwrap());

            // thread::spawn(|| {
            //     Self::handle_connection(stream);
            // });

            Self::handle_connection(stream, self.path.clone());
        }
    }

    // handle connection to the server
    fn handle_connection(mut stream: TcpStream, path: String) {
        // let mut buf = [0u8; 1024];

        // send the file details to the client

        // loop {
        // let n = match stream.read(&mut buf) {
        //     Ok(0) => break,
        //     Ok(n) => n,
        //     Err(_) => 0,
        // };

        // println!("Server recieved {}", String::from_utf8_lossy(&buf[..n]));
        // if stream
        //     .write_all(b"connected to server successfully\n")
        //     .is_err()
        // {
        //     break;
        // }
        // stream.flush().ok();
        // }

        // get the file from the path

        thread::spawn(move || {
            let command = shared::read_message(&mut stream).unwrap();

            if command == "download" {
                // let file_name = read_file_name(&mut stream).unwrap();

                // let file_path = Path::new(&file_name);

                let file_path = Path::new(path.as_str());

                if !file_path.is_file() {
                    error!("File does not exist");
                    shared::write_message(&mut stream, "error").unwrap();
                    return;
                }

                shared::write_message(&mut stream, "ok").unwrap();

                let mut reader = shared::upload::create_reader(&file_path).unwrap();

                loop {
                    let download = client_is_downloading(&mut stream, &mut reader).unwrap();
                    if download.is_none() {
                        return;
                    }
                }
            }
        });
    }
}

fn client_is_downloading(
    stream: &mut TcpStream,
    reader: &mut BufReader<File>,
) -> Result<Option<bool>, Box<dyn Error>> {
    loop {
        let (buf, read_bytes) = shared::upload::read_chunk_from_file(reader).unwrap();

        if read_bytes == 0 {
            // send the eof
            shared::write_eof(stream).unwrap();

            return Ok(None);
        }

        shared::upload::write_chunk(stream, read_bytes, &buf)?;
    }
}
