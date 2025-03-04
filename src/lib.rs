use std::{sync::{mpsc, Arc, Mutex}, thread};

pub struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>
}

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        // Use Arc so that multiple workers can share ownership of the same receiver,
        // and Mutex makes sure only 1 worker receives/requests a new job.
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);
        for i in 0..size {
            workers.push(Worker::new(i, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender: Some(sender) }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());
        for worker in &mut self.workers {
            println!("Shutting down worker {}...", worker.id);
            // If worker thread is None, we do not need to cleanup the thread.
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

impl Worker {
    // TODO: use std::thread::Builder instead
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            // lock() can fail if mutex is in "poisoned" state, which can happen
            // if another thread panics while holding the lock.
            let message = receiver.lock().unwrap().recv();
            match message {
                Ok(job) => {
                    println!("Worker {id} got a job! Executing...");
                    job();
                    println!("Worker {id} finished a job!");
                }
                Err(_) => {
                    println!("Worker {id} disconnected! Shutting down...");
                    break;
                }
            }
        });

        Worker { id, thread: Some(thread) }
    }
}
