use futures::{
    self,
    Future
}; // end use futures

#[tokio::main]
async fn main() {
    blocks().await;
    move_block().await;
} // end main()

async fn blocks() {
    let common_string: String = "foo".to_string();

    // The first async block.
    let future_1 = async {
        println!("{common_string}");
    }; // end first async block.

    // The second async block.
    let future_2 = async {
        println!("{common_string}");
    }; // end second async block.

    let ((), ()) = futures::join!(future_1, future_2);
} // end blocks()

fn move_block() -> impl Future<Output = ()> {
    let common_string: String = "foo".to_string();

    async move {
        println!("Hi! The common string is {common_string}");
    }   
} // end move_block()