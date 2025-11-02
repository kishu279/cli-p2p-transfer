use std::{
    error::Error,
    fs::File,
    io::{Read, Write},
    net::TcpStream,
};

pub fn read_message(stream: &mut TcpStream) -> Result<String, Box<dyn Error>> {
    let mut message_len_bytes = [0; 4];
    stream.read_exact(&mut message_len_bytes).unwrap();
    let message_len = u32::from_be_bytes(message_len_bytes);

    let mut message_bytes = vec![0; message_len as usize];
    stream.read_exact(&mut message_bytes).unwrap();
    let message = String::from_utf8_lossy(&message_bytes);

    Ok(message.to_string())
}

pub fn create_file(file_name: &str) -> Result<File, Box<dyn Error>> {
    let file = std::fs::File::create(&file_name).unwrap();
    Ok(file)
}

pub fn write_message(stream: &mut TcpStream, message: &str) -> Result<(), Box<dyn Error>> {
    let len = message.len() as u32;
    let len_bytes = len.to_be_bytes();
    stream.write_all(&len_bytes)?;

    stream.write_all(message.as_bytes())?;
    Ok(())
}

pub fn write_eof(stream: &mut TcpStream) -> Result<(), Box<dyn Error>> {
    stream.write_all(&[0x00])?;

    Ok(())
}
