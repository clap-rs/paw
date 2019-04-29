use std::io::prelude::*;
use std::net::TcpListener;

#[derive(paw_structopt::StructOpt, structopt::StructOpt)]
struct Args {
    /// Port to listen on.
    #[structopt(short = "p", long = "port", env = "PORT", default_value = "8080")]
    port: u16,
    /// Address to listen on.
    #[structopt(short = "a", long = "address", default_value = "127.0.0.1")]
    address: String,
}

#[paw::main]
fn main(args: Args) -> Result<(), std::io::Error> {
    let listener = TcpListener::bind((args.address.as_str(), args.port))?;
    println!("listening on {}", listener.local_addr()?);
    for stream in listener.incoming() {
        stream?.write(b"hello world!")?;
    }
    Ok(())
}
