use std::{
    error::Error,
    fs::File,
    io::{BufReader, Read, Write},
    net::TcpStream,
    path::Path,
};

pub fn create_reader(file_path: &Path) -> Result<BufReader<File>, Box<dyn Error>> {
    if !file_path.exists() {
        return Err("[-] File does not exist".into());
    }

    if !file_path.is_file() {
        return Err("[-] Path is not a file".into());
    }

    let file = File::open(file_path)?;

    let reader = BufReader::new(file);

    Ok(reader)
}

pub fn read_chunk_from_file(
    reader: &mut BufReader<File>,
) -> Result<([u8; 1024], u32), Box<dyn Error>> {
    let mut buf = [0u8; 1024];
    let read_bytes = reader.read(&mut buf)?;
    let read_bytes = read_bytes as u32;

    Ok((buf, read_bytes))
}

pub fn write_chunk(
    stream: &mut TcpStream,
    read_bytes: u32,
    chunk: &[u8; 1024],
) -> Result<(), Box<dyn Error>> {
    let len_bytes = &1024u32.to_be_bytes();

    if read_bytes < 1024 {
        stream.write_all(&read_bytes.to_be_bytes())?;
    } else {
        stream.write_all(len_bytes)?;
    }

    stream.write_all(chunk)?;

    Ok(())
}
