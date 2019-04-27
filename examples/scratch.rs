use std::io::{self, prelude::*};
use std::net::TcpListener;

struct Args {
    /// Port to listen on.
    port: u16,
    /// Address to listen on.
    address: String,
}

#[paw::main]
fn main(args: Args) -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind((args.address.as_str(), args.port))?;
    println!("listening on {}", listener.local_addr()?);
    for stream in listener.incoming() {
        stream?.write(b"hello world!")?;
    }
    Ok(())
}

impl paw::ParseArgs for Args {
    type Error = Box<dyn std::error::Error>;

    fn parse_args() -> Result<Self, Self::Error> {
        let mut args = std::env::args().skip(1);

        let address = args
            .next()
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "the address arg is missing"))?;

        let port = args
            .next()
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "the port arg is missing"))?
            .parse()?;

        Ok(Self { address, port })
    }
}
