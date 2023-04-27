use futures::select;
use futures::{FutureExt, StreamExt, SinkExt};
use futures::channel::mpsc::{self, Receiver, Sender};


#[tokio::main]
async fn main() {
    traverse_stream().await;
}

async fn traverse_stream() {
    const BUFFER_SIZE: usize = 10;
    let (mut tx, mut rx) : (Sender<u32>, Receiver<u32>) = mpsc::channel(BUFFER_SIZE);
    
    for i in 0..10 {
        select! {
            _ = async { tx.send(i).await }.fuse() => {},
            default => {}
        }
    }

    println!("Everything is sent!");

    drop(tx);

    while let Some(message) = rx.next().await {
        println!("{}", message);
    }
}
