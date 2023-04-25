use futures::executor::block_on;
use std::future::Future;
use std::pin::Pin;
use futures::join;
use futures::future::join_all;

#[tokio::main]
async fn main() {
    let ft = say_hi();
    let mut vc: Vec<Pin<Box<dyn Future<Output = ()>>>> = Vec::new();

    for i in 0..10 {
        vc.push(Box::pin(greet_user(i)));
    } // end for

    block_on(ft);

    join_all(vc).await;

    // for as_fun in vc.into_iter() {
    //     futures::join!(as_fun);
    // } // end for
} // end main()

async fn say_hi() {
    println!("Hi!");
} // end say_hi()

async fn greet_user(num: usize) {
    for _ in 0..10 {
        println!("Hi from async #{num}");
        tokio::time::sleep(tokio::time::Duration::from_millis(30)).await;
    } // end for
} // end greet_user()
