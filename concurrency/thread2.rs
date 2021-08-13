use std::thread;

fn main() {
    let v = vec![1, 2, 3];
    // 把v的所有权移动到闭包里面
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}