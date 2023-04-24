use futures::executor::block_on;

fn main() {
    let ft = say_hi();
    block_on(ft);
} // end main()

async fn say_hi() {
    println!("Hi!");
} // end say_hi()
