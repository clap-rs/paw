use std::net::TcpListener;

struct Args {
    port: usize,
}

impl paw::TryParse for Args {
    type Item = Self;
    type Error = std::io::Error;
    fn try_parse() -> Result<Self::Item, Self::Error> {
        Ok(Self { port: 8080 })
    }
}

#[paw::main]
fn main(args: Args) -> Result<(), std::io::Error> {
    let mut listener = TcpListener::accept(("127.0.0.1", args.port));
    for stream in listener.incoming() {
        stream.write(b"hello world!")?;
    }
}
