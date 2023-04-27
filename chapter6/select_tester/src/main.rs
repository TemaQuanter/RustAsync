use futures::future::FutureExt;
use futures::pin_mut;
use futures::select;

#[tokio::main]
async fn main() {
    prog1().await;
    prog2().await;
} // end main()

async fn first() {
    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
}

async fn second() {

}

async fn prog1() {
    let fut1 = first().fuse();
    let fut2 = second().fuse();

    pin_mut!(fut1, fut2);


    select! {
        () = fut1 => println!("First future is the first!"),
        () = fut2 => println!("Second future is the first!")
    }
}

async fn fun1() -> i32 {
    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
    println!("Fun1 done!");
    17
}

async fn fun2() -> i32 {
    tokio::time::sleep(tokio::time::Duration::from_secs(4)).await;
    println!("Fun2 done!");
    13
}

async fn prog2() {
    let mut fut1 = fun1().fuse();
    let mut fut2 = fun2().fuse();
    let mut total = 0;

    pin_mut!(fut1, fut2);

    loop {
        select! {
            a = fut1 => total += a,
            b = fut2 => total += b,
            complete => break,
            default => {
                println!("Waiting");
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            }
        }
    }

    println!("{}", total);
}