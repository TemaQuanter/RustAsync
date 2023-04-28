use std::{
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll, Waker},
    time::Duration,
};
use futures::FutureExt;
use tokio::time::{sleep, Sleep};

struct ReadyFuture {
    // waker: Arc<Mutex<Option<Waker>>>,
    sleep_future: Sleep,
}

impl ReadyFuture {
    fn new(duration: Duration) -> Self {
        Self {
            // waker: Arc::new(Mutex::new(None)),
            sleep_future: sleep(duration),
        }
    }
}

impl Future for ReadyFuture {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // let mut waker = self.waker.lock().unwrap();

        if self.sleep_future.poll_unpin(cx).is_ready() {
            Poll::Ready(())
        } else {
            // *waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

#[tokio::main]
async fn main() {
    let ready_future = ReadyFuture::new(Duration::from_millis(500));
    println!("Before await");
    ready_future.await;
    println!("After await");
}
