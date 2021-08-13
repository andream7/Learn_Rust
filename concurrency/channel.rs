use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    // 新线程必须有发送端的所有权，才能向通道中发送消息
    thread::spawn(move || {
        let val = String::from("hi");
        // 使用unwarp进行错误处理，触发panic!
        tx.send(val).unwrap();
    });

    // 返回Result<T,E>, 使用unwarp进行处理
    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}