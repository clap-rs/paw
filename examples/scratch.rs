use std::io::prelude::*;
use std::net::TcpListener;

struct Args {
    port: u16,
}

impl paw::ParseArgs for Args {
    type Error = std::io::Error;
    fn try_parse() -> Result<Self, Self::Error> {
        Ok(Self { port: 8080 })
    }
}

#[paw::main]
fn main(args: Args) -> Result<(), std::io::Error> {
    let listener = TcpListener::bind(("127.0.0.1", args.port))?;
    println!("listening on {}", listener.local_addr()?);
    for stream in listener.incoming() {
        stream?.write(b"hello world!")?;
    }
    Ok(())
}
