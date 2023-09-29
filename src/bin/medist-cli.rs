use std::{
    error::Error,
    io::{self, Read, Write},
    net::TcpStream,
    str,
};

const LOCAL_ADDR: &str = "127.0.0.1";
const PORT: &str = "9876";

fn main() -> Result<(), Box<dyn Error>> {
    let mut stream = TcpStream::connect(format!("{}:{}", LOCAL_ADDR, PORT)).unwrap();
    let mut message_buf: Vec<u8> = vec![0; 256];

    loop {
        print!("\nmedist> ");
        io::stdout().flush()?;

        let mut input = String::new();

        std::io::stdin().read_line(&mut input)?;
        input.truncate(input.trim_end().len());

        if input == "q" {
            break;
        }

        stream.write_all(input.as_bytes())?;
        stream.flush()?;

        if stream.read(&mut message_buf).unwrap() == 0 {
            panic!("Server closed");
        }
        
        let s = str::from_utf8(&message_buf).unwrap();
        print!("{}", s);
        io::stdout().flush()?;
    }

    Ok(())
}
