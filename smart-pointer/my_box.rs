use std::ops::Deref;

fn hello(name: &str) {
    println!("hello, {}", name);
}

fn main() {
    let m = MyBox::new(String::from("Rust"));
    // Deref Coercion的体现:
    // &m 的类型是 &MyBox<String>
    // MyBox实现了deref: &MyBox<String> -> &String
    // String也实现了deref：&String -> &str
    hello(&m);
    // 如果没有Deref Coercion这个功能
    hello(&(*m)[..]);
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
// 实现 Deref Trait
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}
