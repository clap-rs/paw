#[paw::main]
fn main(args: paw::Args) {
    for arg in args {
        println!("{:?}", arg);
    }
}
