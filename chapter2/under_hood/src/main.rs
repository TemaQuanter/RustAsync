use futures::FutureExt;
use tokio::time::Sleep;
use std::future::Future;
use std::task::Poll;
use std::time::Duration;
use std::pin::Pin;
use std::sync::{Arc, Mutex};

struct Delay<T> {
    value: Arc<Mutex<Option<T>>>,
    delay: Pin<Box<Sleep>>
} // end struct Delay


#[derive(Debug)]
struct JustStruct {
    name: String,
    age: u8
}


impl<T> Delay<T> {
    fn new(value: T, delay: Duration) -> Self {
        Self {
            value: Arc::new(Mutex::new(Some(value))),
            delay: Box::pin(tokio::time::sleep(delay))
        }
    } // end new()
} // end impl Delay

impl<T> Future for Delay<T> {
    type Output = T;

    fn poll(mut self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        match self.delay.poll_unpin(cx) {
            Poll::Ready(()) => Poll::Ready((*self.value.lock().unwrap()).take().unwrap()),
            Poll::Pending => Poll::Pending
        }
    } // end poll
} // end impl Future for Delay

#[tokio::main]
async fn main() {
    let ftr: Delay<JustStruct> = Delay::new(JustStruct { name: "alsjfa".to_string(), age: 7 }, tokio::time::Duration::from_millis(4500));

    println!("The result is: {:?}", ftr.await);

    let mut a: Option<JustStruct> = Some(JustStruct { name: "lasjfa".to_string(), age: 93 });
    let mut b: JustStruct;

    println!("{:?}", a);

    let b = a.take().unwrap();

    println!("{:?}", a);
    println!("{:?}", b);
} // end main()