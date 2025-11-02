use std::{
    error::Error,
    fs::File,
    io::{Read, Write},
    net::TcpStream,
};

use tracing::info;

pub fn read_file_name(stream: &mut TcpStream) -> Result<String, Box<dyn Error>> {
    // getting file name as bytes
    let mut file_name_len_bytes = [0; 4];
    stream.read_exact(&mut file_name_len_bytes).unwrap();

    // converting the bytes to u32
    let file_name_len: u32 = u32::from_be_bytes(file_name_len_bytes);

    let mut file_name_bytes = vec![0; file_name_len as usize];
    stream.read_exact(&mut file_name_bytes).unwrap();

    let file_name = String::from_utf8(file_name_bytes.to_vec()).unwrap();

    Ok(file_name)
}

pub fn create_file(file_name: &str) -> Result<File, Box<dyn Error>> {
    let file = std::fs::File::create(&file_name).unwrap();
    Ok(file)
}

pub fn read_chunk_from_file(stream: &mut TcpStream) -> Result<Option<Vec<u8>>, Box<dyn Error>> {
    let mut chunk_len_bytes = [0; 4];
    stream.read_exact(&mut chunk_len_bytes).unwrap();
    let chunk_len = u32::from_be_bytes(chunk_len_bytes);

    if chunk_len == 0 {
        info!("[+] File read to the end");
        return Ok(None);
    }

    let mut frame_data = vec![0; chunk_len as usize];
    stream.read_exact(&mut frame_data).unwrap();

    Ok(Some(frame_data))
}

pub fn read_chunk(stream: &mut TcpStream) -> Result<Option<Vec<u8>>, Box<dyn Error>> {
    let mut chunk_len_bytes = [0; 4];
    stream.read_exact(&mut chunk_len_bytes).unwrap();
    let chunk_len = u32::from_be_bytes(chunk_len_bytes);

    if chunk_len == 0 {
        info!("[+] File read to the end");
        return Ok(None);
    }

    let mut frame_data = vec![0; chunk_len as usize];
    stream.read_exact(&mut frame_data).unwrap();

    Ok(Some(frame_data))
}

pub fn write_chunk_to_file(file: &mut File, chunk: &[u8]) {
    file.write_all(chunk).unwrap();
}
