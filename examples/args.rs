use std::error::Error;
use std::io::prelude::*;
use std::net::TcpListener;

/// To run this example (e.g. with port 8080):
///
/// ```sh
/// $ cargo run --example new_type -- --port 8080
/// ```

#[paw::main]
fn main(args: paw::Args) -> Result<(), Box<dyn Error>> {
    // Rust reads the arguments to the process as an env::Args which is
    // an iterator over strings. It contains the arguments as strings if
    // they are separated by space. Ensure you give the argument as --port 8080
    // and not --port=8080 in this case.

    let mut iter = args.0.skip_while(|e| e != "--port").skip(1);
    let port = iter
        .next()
        .ok_or_else(|| paw::ArgNotFoundError { arg: "port".into() })?;
    let listener = TcpListener::bind(("127.0.0.1", port.parse()?))?;
    println!("listening on {}", listener.local_addr()?);
    for stream in listener.incoming() {
        stream?.write(b"hello world!")?;
    }
    Ok(())
}
