//! Used for sending jobs to a pool of threads  

use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

/// A job is a pointer to a function on the heap
type Job = Box<dyn FnOnce() + Send + 'static>;

/// Each thread either receives a new job or a terminate message each loop 
enum Message {
    NewJob(Job),
    Terminate,
}

/// Creates and manages a given number of threads  
/// Code borrowed from doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html
pub struct ThreadPool {
    /// Handles to each thread
    workers:  Vec<Worker>,
    /// Used to send jobs to the threads
    sender:   mpsc::Sender<Message>,
    /// Send a sync signal to one of the threads
    sync_out: Arc<Mutex<mpsc::Sender<usize>>>,
    /// Get back a sync signal from one of the threads
    sync_in:  mpsc::Receiver<usize>,
}

impl ThreadPool {
    /// Returns a new ThreadPool with the given number of threads
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let mut workers = Vec::with_capacity(size);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let (sync_out, sync_in) = mpsc::channel();
        // This makes first call to wait non-blocking
        sync_out.send(0).unwrap();
        let sync_out = Arc::new(Mutex::new(sync_out));

        for id in 0..size {
            println!("Starting worker {}", id);
            workers.push(Worker::new(id, receiver.clone()));
        }

        ThreadPool { workers, sender, sync_out, sync_in }
    }

    /// Enqueue a job
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);

        self.sender.send(Message::NewJob(job)).unwrap();
    }

    /// This will wait for all jobs enqueued since last call to wait to finish  
    /// Returns the number of times wait got called starting from 0  
    /// NOTE: The first call to wait doesn't block to allow a loop that always has work enqueued  
    /// # Example
    /// 
    /// ```
    /// use nerdee_engine::thread_pool::ThreadPool;
    /// use std::thread;
    /// // Create a new thread pool with 8 threads
    /// let pool = ThreadPool::new(8);
    /// for i in 0..5 {
    ///     for i in 0..10 {
    ///         pool.execute( move || {
    ///             println!("I got {}", i);
    ///             thread::sleep_ms(10);
    ///         });
    ///     }
    ///     println!("Loop count: {}", pool.wait());
    ///     
    /// }
    /// ```
    /// The first time through the loop wait() doesn't block so more jobs can get enqueued  
    /// The second time through the loop wait() will wait for the first 10 jobs to finish  
    /// then start enqueueing more jobs before the second 10 jobs finish, that way it doesn't stall  
    pub fn wait(&self) -> usize {
        let frame = self.sync_in.recv().unwrap();
        let sync_out = self.sync_out.clone();
        self.execute(move || {
            let sync_out = sync_out.lock().unwrap();
            sync_out.send(frame + 1).unwrap();
        });

        frame
    }
}

impl Drop for ThreadPool {
    /// Waits for all jobs to finish then gracefully shuts down each thread
    fn drop(&mut self) {

        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        for worker in &mut self.workers {
            println!("Stopping worker {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

/// A worker that runs on it's own thread  
/// Has an id and a handle
struct Worker {
    id:     usize,
    thread: Option<thread::JoinHandle<()>>
}

impl Worker {
    /// Spawns a new worker
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();

            match message {
                Message::NewJob(job) => {
                    job();
                }
                Message::Terminate => {
                    break;
                }
            }
        });

        let thread = Some(thread);

        Worker { id, thread }
    }
} 