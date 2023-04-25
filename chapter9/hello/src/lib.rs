use std::{
    sync::{
        mpsc,
        mpsc::{Receiver, Sender},
        Arc, Mutex,
    },
    thread,
    thread::JoinHandle,
}; // end use std::thread


type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<Sender<Job>>,
} // end ThreadPool

impl ThreadPool {
    /// This function creates a new ThreadPool.
    ///
    /// # Panics
    ///
    /// The `new` panics if the `size` equals to 0.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver): (Sender<Job>, Receiver<Job>) = mpsc::channel();

        let receiver: Arc<Mutex<Receiver<Job>>> = Arc::new(Mutex::new(receiver));

        let mut workers: Vec<Worker> = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        } // end for

        ThreadPool {
            workers,
            sender: Some(sender),
        }
    } // end new()

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce(),
        F: Send + 'static,
    {
        let job: Job = Box::new(f);

        self.sender.as_ref().unwrap().send(job).unwrap();
    } // end execute()
} // end impl ThreadPool

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            } // end if
        } // end for
    } // end drop()
} // end impl Drop for ThreadPool

struct Worker {
    id: usize,
    thread: Option<JoinHandle<()>>,
} // end struct Worker

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Worker {
        let thread: JoinHandle<()> = thread::spawn(move || loop {
            let result: Result<Job, _> = receiver.lock().unwrap().recv();

            match result {
                Ok(job) => {
                    println!("Worker {id} got a job, executing...");

                    job();
                }
                Err(_) => {
                    println!("Worker {id} disconnected, shutting down.");

                    break;
                }
            } // end match
        }); // end thread

        Worker {
            id,
            thread: Some(thread),
        } // end Worker
    } // end new()
} // end impl Worker
