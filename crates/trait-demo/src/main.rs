// fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
//     let mut largest = list[0];

//     for &item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

// fn main() {
//     let number_list = vec![34, 50, 25, 100, 65];

//     let result = largest(&number_list);
//     println!("The largest number is {}", result);

//     let char_list = vec!['y', 'm', 'a', 'q'];

//     let result = largest(&char_list);
//     println!("The largest char is {}", result);
// }

// 调用方法需要引入特征
// 在一些场景中，使用 as 关键字做类型转换会有比较大的限制，因为你想要在类型转换上拥有完全的控制，例如处理转换错误，那么你将需要 TryInto：

// use std::convert::TryInto;

// fn main() {
//     let a: i32 = 10;
//     let b: u16 = 100;

//     let b_ = b.try_into().unwrap();
//     // if a < b {
//     //     |        -   ^ expected `i32`, found `u16`
//     //     |        expected because this is `i32`

//     if a < b_ {
//         println!("Ten is less than one hundred.");
//     }
// }

// use std::ops::Add;

// // 为Point结构体派生Debug特征，用于格式化输出
// #[derive(Debug)]
// struct Point<T: Add<T, Output = T>> {
//     //限制类型T必须实现了Add特征，否则无法进行+操作。
//     x: T,
//     y: T,
// }

// impl<T: Add<T, Output = T>> Add for Point<T> {
//     type Output = Point<T>;

//     fn add(self, p: Point<T>) -> Point<T> {
//         Point {
//             x: self.x + p.x,
//             y: self.y + p.y,
//         }
//     }
// }

// fn add<T: Add<T, Output = T>>(a: T, b: T) -> T {
//     a + b
// }

// fn main() {
//     let p1 = Point {
//         x: 1.1f32,
//         y: 1.1f32,
//     };
//     let p2 = Point {
//         x: 2.1f32,
//         y: 2.1f32,
//     };
//     println!("{:?}", add(p1, p2));

//     let p3 = Point { x: 1i32, y: 1i32 };
//     let p4 = Point { x: 2i32, y: 2i32 };
//     println!("{:?}", add(p3, p4));
// }

// #![allow(dead_code)]

// use std::fmt;
// use std::fmt::Display;

// #[derive(Debug, PartialEq)]
// enum FileState {
//     Open,
//     Closed,
// }

// #[derive(Debug)]
// struct File {
//     name: String,
//     data: Vec<u8>,
//     state: FileState,
// }

// impl Display for FileState {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match *self {
//             FileState::Open => write!(f, "OPEN"),
//             FileState::Closed => write!(f, "CLOSED"),
//         }
//     }
// }

// impl Display for File {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "<{} ({})>", self.name, self.state)
//     }
// }

// impl File {
//     fn new(name: &str) -> File {
//         File {
//             name: String::from(name),
//             data: Vec::new(),
//             state: FileState::Closed,
//         }
//     }
// }

// fn main() {
//     let f6 = File::new("f6.txt");
//     //...
//     println!("{:?}", f6);
//     println!("{}", f6);
// }

// // 在 Rust 中，有两个self，一个指代当前的实例对象，一个指代特征或者方法类型的别名：
// trait Draw {
//     fn draw(&self) -> Self;
// }

// #[derive(Clone)]
// struct Button;
// impl Draw for Button {
//     fn draw(&self) -> Self {
//         // self指代的就是当前的实例对象，也就是 button.draw() 中的 button 实例，Self 则指代的是 Button 类型。
//         return self.clone();
//     }
// }

// fn main() {
//     let button = Button;
//     let newb = button.draw();
// }

// trait Animal {
//     fn baby_name() -> String;
// }

// struct Dog;

// impl Dog {
//     fn baby_name() -> String {
//         String::from("Spot")
//     }
// }

// impl Animal for Dog {
//     fn baby_name() -> String {
//         String::from("puppy")
//     }
// }

// fn main() {
//     println!("A baby dog is called a {}", Dog::baby_name());
//     // println!("A baby dog is called a {}", Animal::baby_name()); // ^^ cannot call associated function of trait

//     // as 完全限定语法
//     println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
// }


// use std::fmt::Display;

// trait OutlinePrint: Display {
//     fn outline_print(&self) {
//         let output = self.to_string();
//         let len = output.len();
//         println!("{}", "*".repeat(len + 4));
//         println!("*{}*", " ".repeat(len + 2));
//         println!("* {} *", output);
//         println!("*{}*", " ".repeat(len + 2));
//         println!("{}", "*".repeat(len + 4));
//     }
// }

use std::fmt;

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}
