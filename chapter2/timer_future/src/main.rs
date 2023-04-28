use timer_future::executors::new_executor_and_spawner;
use timer_future::t_future::TimerFuture;
use std::time::Duration;

fn main() {
    let (executor, spawner) = new_executor_and_spawner();

    spawner.spawn(async {
        println!("Start");
        TimerFuture::new(Duration::new(2, 0)).await;
        println!("Finish");
    }); // end spawner.spawn()

    drop(spawner);

    executor.run();
} // end main()