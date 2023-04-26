use std::future::Future;
use std::task::Poll;
use std::pin::Pin;

struct SimpleStruct {
    value: u32
} // end struct SimpleStruct


impl Future for SimpleStruct {
    type Output = u32;

    fn poll(self: Pin<&mut Self>, _cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        Poll::Ready(self.value)
    } // end poll()
} // end impl Future for SimpleStruct

impl SimpleStruct {
    fn new(value: u32) -> Self {
        Self {
            value
        } // end Self
    } // end new()
} // end impl SimpleStruct


#[tokio::main]
async fn main() {
    let simple_struct: SimpleStruct = SimpleStruct::new(7);

    println!("SimpleStruct: {}", simple_struct.await);
    // println!("The result is: {}", ftr.await);
} // end main()
