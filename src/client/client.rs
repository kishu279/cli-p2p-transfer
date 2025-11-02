pub struct FileClient {
    address: String,
    port: u16,
    code: Option<u16>,
    connection: TcpStream,
    download_path: String,
}
