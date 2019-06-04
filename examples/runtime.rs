#![feature(async_await)]

#[paw::main]
#[runtime::main]
async fn main(args: paw::Args) {
    for (i, arg) in args.enumerate() {
        println!("#{:?}: {:?}", i, arg);
    }
    runtime::spawn(async move {
        dbg!("hello");
    })
    .await;
}
