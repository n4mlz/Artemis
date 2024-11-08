use std::sync::{mpsc, Arc, Mutex};
use std::thread;

enum Message {
    NewJob(Job),
    Terminate,
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    tx: mpsc::Sender<Message>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (tx, rx) = mpsc::channel();
        let rx = Arc::new(Mutex::new(rx));
        let workers = (0..size).map(|_| Worker::new(Arc::clone(&rx))).collect();

        ThreadPool { workers, tx }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.tx.send(Message::NewJob(job)).unwrap();
    }
}

struct Worker {
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(rx: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let msg = rx.lock().unwrap().recv().unwrap();

            match msg {
                Message::NewJob(job) => {
                    job();
                }
                Message::Terminate => {
                    break;
                }
            }
        });

        Worker {
            thread: Some(thread),
        }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for _ in &mut self.workers {
            self.tx.send(Message::Terminate).unwrap();
        }
        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
