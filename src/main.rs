use clap::{Arg, Command};

pub mod server;
pub use server::*;

pub mod client;
pub use client::*;

fn main() {
    // Cli argunments passing using clap
    let matches = Command::new("P2P File Transfer")
        .version("0.1.0")
        .about("A simple file transfer program using TCP protocol.")
        // host
        .arg(
            Arg::new("host")
                .short('s')
                .long("host")
                .num_args(0)
                .help("want to be host"),
        )
        .arg(
            Arg::new("port")
                .short('p')
                .long("port")
                .num_args(1)
                .value_parser(clap::value_parser!(u16))
                .default_value("8080")
                .help("specify the port"),
        )
        .arg(
            Arg::new("address")
                .short('i')
                .long("address")
                .default_value("127.0.0.1")
                .help("which host you want to connect on the local network"),
        )
        .arg(
            Arg::new("code")
                .short('c')
                .long("code")
                .value_parser(clap::value_parser!(u16))
                .num_args(1)
                .help("code"),
        )
        .arg(
            Arg::new("location")
                .short('l')
                .long("path")
                .num_args(1)
                .help("set the location for upload/download"),
        )
        .get_matches();

    // get the values from the cli arguments
    let host_or_not = matches.get_flag("host"); // is the user want to be host?
    let port = matches.get_one::<u16>("port").unwrap(); // get the port
    let address = matches.get_one::<String>("address").unwrap(); // get the ip address
    let code: Option<u16> = matches.get_one("code").copied();
    let path: &String = matches.get_one("location").unwrap();

    println!("port : {:?}", port);
    println!("address : {:?}", address);
    println!("code : {:?}", code);
    println!("path : {:?}", path);

    match host_or_not {
        true => {
            let hosting = FileServer::new(address.to_string(), *port, None, code);

            hosting.run();

            // sending 1024 bytes on each iteration to each client
        }
        false => {
            // println!("Client");

            // // connect to the server using the ip address and port
            // let mut stream = TcpStream::connect(format!("{}:{}", address, port)).unwrap();

            // let _ = stream.write(b"Hello World!").unwrap();
            // let mut buffer = [0; 1024];
            // let response = stream.read(&mut buffer).unwrap();

            // println!(
            //     "Response after sending to the server {}",
            //     String::from_utf8_lossy(&buffer[..response])
            // );

            //
            let client = FileClient::new(address.to_string(), *port, code, path.to_string());

            FileClient::run(&client);
        }
    }
}
