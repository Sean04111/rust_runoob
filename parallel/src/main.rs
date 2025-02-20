// this is parallel in rust =D

use std::future::Future;
use futures::executor::block_on;

async fn hello(){
    println!("hello");
}

//.await method can only be used in an async function, to wait another future finished

async fn world (){
    hello().await;
    println!("world");
}

async fn foo() -> u8 {5}

// here
fn bar() -> impl Future<Output = u8>{
    async {
        let x:u8 = foo().await;
        x + 5
    }
}

fn main() {
    let future2 = world();

    block_on(future2);
}

