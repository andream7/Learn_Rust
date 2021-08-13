use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("{}", i);
            // thread::slepp 会导致当前线程暂停执行
            thread::sleep(Duration::from_millis(1));
        }
    });
    /*
    阻塞主线程的执行，直到handle对应的线程执行结束
    */
    handle.join().unwrap();

    for i in 1..5 {
        println!("{}", i);
            thread::sleep(Duration::from_millis(1));
    }
}