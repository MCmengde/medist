use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

const LOCAL_ADDR: &str = "127.0.0.1";
const PORT: &str = "9876";

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind(format!("{}:{}", LOCAL_ADDR, PORT))?;

    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                handle_client(&mut _stream);
            }
            Err(_) => {
                continue;
            }
        }
    }
    Ok(())
}

fn handle_client(stream: &mut TcpStream) {
    println!("Hello Client: {}", stream.peer_addr().unwrap());
    let mut buffer = vec![0u8; 256];
    loop {
        match stream.read(&mut buffer) {
            Ok(read_bytes) => {
                if read_bytes > 0 {
                    stream.write_all(&buffer[..read_bytes]).unwrap();
                } else {
                    break;
                }
            }
            Err(_) => {break;}
        }
    }
}
