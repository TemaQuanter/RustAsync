use std::future::Future;
use std::sync::{Arc, Mutex};
use std::task::{Context, Waker, Poll};
use std::time::{Duration, Instant};
use std::pin::Pin;

struct CustomTimer {
    dead_line: Instant,
    waker: Arc<Mutex<Option<Waker>>>
}

impl CustomTimer {
    fn new(delay: Duration) -> Self {
        let dead_line: Instant = Instant::now() + delay;
        
        Self {
            dead_line,
            waker: Arc::new(Mutex::new(None))
        }
    }
}

impl Future for CustomTimer {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut cur_waker = self.waker.lock().unwrap();
        // Check if the object has run out of time.
        if Instant::now() >= self.dead_line {
            Poll::Ready(())
        } else {
            *cur_waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

#[tokio::main]
async fn main() {
    let timer: CustomTimer = CustomTimer::new(Duration::from_millis(3));
    println!("The timer is started!");
    timer.await;
    println!("Beep Beep!");
} // end main()
