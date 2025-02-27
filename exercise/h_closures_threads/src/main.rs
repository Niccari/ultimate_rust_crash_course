// Silence some warnings so they don't distract from the exercise.
#![allow(dead_code, unused_imports, unused_variables)]
use crossbeam::channel;
use std::thread;
use std::time::Duration;

fn expensive_sum(v: Vec<i32>) -> i32 {
    pause_ms(500);
    println!("Child thread: just about finished");

    v.iter()
        .filter(|&x| x % 2 == 0)
        .map(|x| x * x)
        .sum()
}

fn pause_ms(ms: u64) {
    thread::sleep(Duration::from_millis(ms));
}

fn main() {
    /*
    let my_vector = vec![2, 5, 1, 0, 4, 3];

    let handle = thread::spawn(|| {
        expensive_sum(my_vector)
    });

    for letter in vec!["a", "b", "c", "d", "e", "f"] {
        println!("Main thread: Letter {}", letter);
        pause_ms(200);
    }

    // 3. Let's retrieve the value returned by the child thread once it has exited.  Using the
    // `handle` variable you stored the join handle in earlier, call .join() to wait for the thread
    // to exit with a `Result<i32, Err>`.  Get the i32 out of the result and store it in a `sum`
    // variable.  Uncomment the println.  If you did 1a and 1b correctly, the sum should be 20.
    //
    let sum = handle.join()
        .expect("Cannot handle expensive sum.");
    println!("The child thread's expensive sum is {}", sum);
    */

    // Time for some fun with threads and channels!  Though there is a primitive type of channel
    // in the std::sync::mpsc module, I recommend always using channels from the crossbeam crate,
    // which is what we will use here.
    //
    // 4. Uncomment the block comment below (Find and remove the `/*` and `*/`).  Examine how the
    // flow of execution works.  Once you understand it, alter the values passed to the `pause_ms()`
    // calls so that both the "Thread B" outputs occur before the "Thread A" outputs.

    /*
    let (tx, rx) = channel::unbounded();
    // Cloning a channel makes another variable connected to that end of the channel so that you can
    // send it to another thread.
    let tx2 = tx.clone();

    let handle_a = thread::spawn(move || {
        pause_ms(0);
        tx2.send("Thread A: 1").unwrap();
        pause_ms(200);
        tx2.send("Thread A: 2").unwrap();
    });

    pause_ms(100); // Make sure Thread A has time to get going before we spawn Thread B

    let handle_b = thread::spawn(move || {
        pause_ms(0);
        tx.send("Thread B: 1").unwrap();
        pause_ms(200);
        tx.send("Thread B: 2").unwrap();
    });

    for msg in rx {
        println!("Main thread: Received {}", msg);
    }

    handle_a.join().unwrap();
    handle_b.join().unwrap();
    */

    // Challenge: Make two child threads and give them each a receiving end to a channel.  From the
    // main thread loop through several values and print each out and then send it to the channel.
    // On the child threads print out the values you receive. Close the sending side in the main
    // thread by calling `drop(tx)` (assuming you named your sender channel variable `tx`).  Join
    // the child threads.

    let (tx, rx) = channel::unbounded();
    let (tx2, rx2) = channel::unbounded();
    let handle_a = thread::spawn(move || {
        for msg in rx {
            println!("Child1 thread: Received {}", msg);
        }
    });

    pause_ms(100); // Make sure Thread A has time to get going before we spawn Thread B

    let handle_b = thread::spawn(move || {
        for msg in rx2 {
            println!("Child2 thread: Received {}", msg);
        }
    });

    tx.send("c1: aaa").unwrap();
    tx2.send("c2: aaa").unwrap();
    tx.send("c1: bbb").unwrap();
    tx2.send("c2: bbb").unwrap();

    drop(tx);
    drop(tx2);

    handle_a.join().unwrap();
    handle_b.join().unwrap();

    println!("Main thread: Exiting.")
}
