use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

struct Queue {
    length: u32,
    first_half: Vec<u32>,
    second_half: Vec<u32>,
}

impl Queue {
    fn new() -> Self {
        Queue {
            length: 10,
            first_half: vec![1, 2, 3, 4, 5],
            second_half: vec![6, 7, 8, 9, 10],
        }
    }
}

fn send_tx(q: Arc<Mutex<Queue>>, tx: Arc<Mutex<mpsc::Sender<u32>>>) {
    let qc1 = Arc::clone(&q);
    let qc2 = Arc::clone(&q);
    let tx1 = Arc::clone(&tx);
    let tx2 = Arc::clone(&tx);

    thread::spawn(move || {
        let queue = qc1.lock().unwrap();
        let sender = tx1.lock().unwrap();
        for val in &queue.first_half {
            println!("sending {:?}", val);
            sender.send(*val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let queue = qc2.lock().unwrap();
        let sender = tx2.lock().unwrap();
        for val in &queue.second_half {
            println!("sending {:?}", val);
            sender.send(*val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
}

fn main() {
    let (tx, rx) = mpsc::channel();
    let queue = Arc::new(Mutex::new(Queue::new()));
    let sender = Arc::new(Mutex::new(tx));
    let queue_length = {
        let queue = queue.lock().unwrap();
        queue.length
    };

    send_tx(Arc::clone(&queue), Arc::clone(&sender));

    let mut total_received: u32 = 0;
    let mut received_1 = 0;
    let mut received_2 = 0;

    loop {
        let received = rx.try_recv();
        match received {
            Ok(val) => {
                println!("Got: {}", val);
                total_received += 1;
                if total_received <= queue_length / 2 {
                    received_1 += 1;
                } else {
                    received_2 += 1;
                }
            }
            Err(_) => {
                if received_1 + received_2 == queue_length {
                    break;
                }
            }
        }
    }

    println!("total numbers received: {}", total_received);
    assert_eq!(total_received, queue_length)
}