use std::error::Error;
use std::io::{self, prelude::*};
use std::net::TcpListener;

/// To run this example do:
/// ```sh
/// $ cargo run --example args -- localhost 8080
/// ```

#[paw::main]
fn main(args: paw::Args) -> Result<(), Box<dyn Error>> {
    let mut args = args.skip(1);

    let host = args
        .next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "the host argument is missing"))?;

    let port = args
        .next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "the port argument is missing"))?
        .parse()?;

    let listener = TcpListener::bind((host.as_str(), port))?;
    println!("listening on {}", listener.local_addr()?);

    for stream in listener.incoming() {
        stream?.write(b"hello world!")?;
    }
    Ok(())
}
