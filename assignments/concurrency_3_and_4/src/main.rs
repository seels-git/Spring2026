use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;
use rand::Rng;

// ─────────────────────────────────────────────
//  ASSIGNMENT 3: Thread Pool
// ─────────────────────────────────────────────

enum Message {
    NewJob(Job),
    Terminate,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

impl ThreadPool {
    fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers...");

        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();

            match message {
                Message::NewJob(job) => {
                    println!("Worker {id} received a job; executing...");
                    job();
                }
                Message::Terminate => {
                    println!("Worker {id} received terminate signal; shutting down.");
                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

fn run_assignment3() {
    println!("\n========== ASSIGNMENT 3: Thread Pool ==========\n");

    let pool = ThreadPool::new(4);

    for i in 1..=10 {
        pool.execute(move || {
            println!("  >> Worker starting task {i}");
            thread::sleep(Duration::from_millis(500));
            println!("  >> Worker completed task {i}");
        });
    }

    println!("Main thread waiting for tasks to complete...");
    // pool drops here, triggering clean shutdown
}

// ─────────────────────────────────────────────
//  ASSIGNMENT 4: Producer-Consumer
// ─────────────────────────────────────────────

const TERMINATION_SIGNAL: i32 = -1;

fn producer(id: usize, tx: mpsc::Sender<i32>, item_count: usize) {
    let mut rng = rand::rng();

    for _ in 0..item_count {
        let value: i32 = rng.random_range(1..=100);
        println!("Producer {id} sending: {value}");
        tx.send(value).unwrap();
        thread::sleep(Duration::from_millis(50));
    }

    println!("Producer {id} finished.");
}

fn consumer(id: usize, rx: Arc<Mutex<mpsc::Receiver<i32>>>) {
    loop {
        let result = rx.lock().unwrap().recv();

        match result {
            Ok(value) => {
                if value == TERMINATION_SIGNAL {
                    println!("Consumer {id} received termination signal. Exiting.");
                    break;
                }
                let squared = value * value;
                println!("Consumer {id} processed: {value} → squared = {squared}");
                thread::sleep(Duration::from_millis(30));
            }
            Err(_) => {
                println!("Consumer {id}: channel closed, shutting down.");
                break;
            }
        }
    }
}

fn run_assignment4() {
    println!("\n========== ASSIGNMENT 4: Producer-Consumer ==========\n");

    const ITEMS_PER_PRODUCER: usize = 10;
    const NUM_PRODUCERS: usize = 2;
    const NUM_CONSUMERS: usize = 3;

    let (tx, rx) = mpsc::channel::<i32>();
    let rx = Arc::new(Mutex::new(rx));

    let mut handles = Vec::new();

    for id in 1..=NUM_PRODUCERS {
        let tx_clone = tx.clone();
        let handle = thread::spawn(move || {
            producer(id, tx_clone, ITEMS_PER_PRODUCER);
        });
        handles.push(handle);
    }

    drop(tx);

    for id in 1..=NUM_CONSUMERS {
        let rx_clone = Arc::clone(&rx);
        let handle = thread::spawn(move || {
            consumer(id, rx_clone);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("\nAll items have been produced and consumed!");
}

// ─────────────────────────────────────────────
//  MAIN
// ─────────────────────────────────────────────

fn main() {
    run_assignment3();
    run_assignment4();
}