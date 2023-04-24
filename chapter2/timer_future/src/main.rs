use futures::{
    future::{BoxFuture, FutureExt},
    task::{ArcWake, waker_ref},
};

use std::{
    future::Future,
    sync::mpsc::{sync_channel, Receiver, SyncSender},
    sync::{Arc, Mutex},
    task::Context,
    time::Duration,
};

use timer_future::TimerFuture;

struct Executor {
    ready_queue: Receiver<Arc<Task>>,
} // end struct Executor

#[derive(Clone)]
struct Spawner {
    task_sender: SyncSender<Arc<Task>>,
} // end struct Spawner

struct Task {
    future: Mutex<Option<BoxFuture<'static, ()>>>,
    task_sender: SyncSender<Arc<Task>>,
} // end struct Task

fn new_executor_and_spawner() -> (Executor, Spawner) {
    const MAX_QUEUED_TASKS: usize = 10_000;
    let (task_sender, ready_queue) = sync_channel(MAX_QUEUED_TASKS);
    (Executor { ready_queue }, Spawner { task_sender })
} // end new_executor_and_spawner()

impl Spawner {
    fn spawn(&self, future: impl Future<Output = ()> + 'static + Send) {
        let future = future.boxed();
        let task = Arc::new(Task {
            future: Mutex::new(Some(future)),
            task_sender: self.task_sender.clone()
        });
        self.task_sender.send(task).expect("Too many tasks queued");
    } // end spawn()
} // end impl Spawner

impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        let cloned = arc_self.clone();
        arc_self
            .task_sender
            .send(cloned)
            .expect("Too many tasks queued");
    } // end wake_by_ref()
} // end impl ArcWake

impl Executor {
    fn run(&self) {
        while let Ok(task) = self.ready_queue.recv() {
            let mut future_slot = task.future.lock().unwrap();
            if let Some(mut future) = future_slot.take() {
                let waker = waker_ref(&task);
                let context = &mut Context::from_waker(&waker);
                if future.as_mut().poll(context).is_pending() {
                    *future_slot = Some(future);
                } // end if
            } // end if
        } // end while
    } // end run()
} // end impl Executor

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