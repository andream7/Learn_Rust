use std::sync::Mutex;

fn main() {
    // 5就是我们要保护的数据
    let m = Mutex::new(5);  

    {
        // lock获取锁（阻塞当前线程直到获得锁为止）
        // 返回MutexGuard
        let mut num = m.lock().unwrap();  

        *num = 6;  
    } // 实现了drop Trait，在此处会自动解锁

    println!("m = {:?}", m);
}