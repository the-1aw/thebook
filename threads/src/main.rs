use std::{thread, time::Duration};

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
}
