#![feature(async_await, await_macro)]

#[paw::main]
#[runtime::main]
async fn main(args: paw::Args) {
    for (i, arg) in args.enumerate() {
        println!("#{:?}: {:?}", i, arg);
    }
}
