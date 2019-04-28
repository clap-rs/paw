#![feature(async_await, await_macro)]

async fn print_args(args: paw::Args) {
    for (i, arg) in args.enumerate() {
        println!("#{:?}: {:?}", i, arg);
    }
}

#[paw::main]
#[runtime::main]
async fn main(args: paw::Args) {
    await!(print_args(args));
}
