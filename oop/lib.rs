pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    // dyn 表示Box中的元素都实现了Draw Trait
    // 只要实现了 Draw Trait 就可以放在同一个Vec里面
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
// 泛型的实现 ----------------
// 泛型中的类型一旦确定，就不能改变
// 也就是说泛型的Vec中只能放一种类型
// pub struct Screen<T: Draw> {
//     pub components: Vec<T>,
// }

// impl<T> Screen<T> 
// where T:Draw,
// {
//     pub fn run(&self) {
//         for component in self.components.iter() {
//             component.draw();
//         }
//     }
// }

// 定义一个struct
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {

    }
}

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };
}