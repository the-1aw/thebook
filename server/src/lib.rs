use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker {
    id: usize,
    handle: Option<thread::JoinHandle<()>>,
}

impl Worker {
    pub fn new(id: usize, rx: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let handle = Some(thread::spawn(move || loop {
            let job = rx.lock().unwrap().recv();
            match job {
                Ok(job) => {
                    println!("Worker {id} got a job; running...");
                    job();
                    println!("Worker {id} done; idle...");
                }
                Err(_) => {
                    println!("Channel closed; disconnecting...");
                    break;
                }
            }
        }));
        Worker { id, handle }
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    transmitter: Option<mpsc::Sender<Job>>,
}

impl ThreadPool {
    /// Create a new Threadpool.
    ///
    /// nb_worker is the number of worker in the pool.
    ///
    /// # Panics
    ///
    /// `ThreadPool::new` will panic if nb_worker is 0.
    pub fn new(nb_worker: usize) -> ThreadPool {
        assert!(nb_worker > 0);
        let mut workers = Vec::with_capacity(nb_worker);
        let (tx, rx) = mpsc::channel();
        let rx = Arc::new(Mutex::new(rx));

        for id in 0..nb_worker {
            workers.push(Worker::new(id, Arc::clone(&rx)));
        }
        ThreadPool {
            workers,
            transmitter: Some(tx),
        }
    }

    /// Queue new task `func` to be executed by first available worker.
    pub fn execute<F>(&self, func: F)
    where
        F: FnOnce() + Send + 'static,
    {
        if let Some(tx) = &self.transmitter {
            tx.send(Box::new(func)).unwrap();
        }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.transmitter.take());
        for worker in &mut self.workers {
            println!("Shutting down worker {}!", worker.id);
            if let Some(handle) = worker.handle.take() {
                handle.join().unwrap();
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[should_panic]
    fn cant_create_empty_pool() {
        ThreadPool::new(0);
    }
}
