use std::{error::Error, io::{self, Write}, net::TcpStream};

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

        print!("{}", input);
    }

    Ok(())
}
