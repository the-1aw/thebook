use std::{sync::mpsc, thread, time::Duration};

fn spawn_thread_explicit_move() -> thread::JoinHandle<Vec<usize>> {
    let mut v = vec![];
    // Here we return a clone of v
    // the closure does can't infer that v needs to be move
    // so we need to force the move using the move keyword
    thread::spawn(move || {
        for i in 1..=5 {
            println!("hi numver {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
            v.push(i);
        }
        v.clone()
    })
}

fn spawn_thread_implicit_move() -> thread::JoinHandle<Vec<usize>> {
    let mut v = vec![];

    // Here we return v
    // this can only work if ownerships moves since v does not impl Copy
    // the closure infer that v needs to be moved
    // no need for move keyword
    thread::spawn(|| {
        for i in 1..=5 {
            println!("hi numver {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
            v.push(i);
        }
        v
    })
}

fn spawn_thread_send_stuff(tx: mpsc::Sender<String>) {
    thread::spawn(move || {
        let stuff = vec!["send".to_string(), "from".to_string(), "string".to_string()];
        for s in stuff {
            tx.send(s).unwrap();
        }
    });
}

fn main() {
    let i_handle = spawn_thread_implicit_move();
    let e_handle = spawn_thread_explicit_move();
    for i in 1..=20 {
        println!("hi number {i} from main thread!");
        thread::sleep(Duration::from_millis(1));
    }
    let e_v = e_handle.join().unwrap();
    let i_v = i_handle.join().unwrap();
    assert_eq!(vec![1, 2, 3, 4, 5], e_v);
    assert_eq!(vec![1, 2, 3, 4, 5], i_v);

    let (tx, rx) = mpsc::channel();
    // clone to allow multiple producers
    spawn_thread_send_stuff(tx.clone());
    // move so channel is closed at the end of spawn_thread_send_stuff
    spawn_thread_send_stuff(tx);
    let mut rx_buffer = Vec::new();
    for receive in rx {
        println!("got: {receive}");
        rx_buffer.push(receive);
    }
    assert_eq!(rx_buffer.len(), 6);
}
