use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}
impl ThreadPool {
    pub fn execute<Func>(&self, func: Func)
    where
        Func: FnOnce() + Send + 'static,
    {
        let job = Box::new(func);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
    /// 创建线程池。
    ///
    /// # Panics
    ///
    /// `new` 函数在 size 为 0 时会 panic。
    pub fn new(size: usize) -> Self {
        assert!(size != 0);
        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        Self {
            workers,
            sender: Some(sender),
        }
    }
}
impl Drop for ThreadPool {
    fn drop(&mut self) {
        self.sender.take();
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            if let Some(worker) = worker.thread.take() {
                worker.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}
impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();

            if let Ok(job) = message {
                println!("Worker {id} got a Job!");
                job();
            } else {
                println!("Worker {id} is disconnected! Will shutting down.");
                break;
            }
        });
        Self {
            id,
            thread: Some(thread),
        }
    }
}

type Job = Box<dyn FnOnce() + Send>;
