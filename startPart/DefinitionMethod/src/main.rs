#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
/*
* impl:
* 为一个类型实现一些功能。
* impl关键字主要用于定义类型的实现。固有实现是独立的，而特征实现用于实现类型的特征或其他特征。
* 函数和常量都可以在实现中定义。块中定义的函数 impl可以是独立的，
* 这意味着它可以像Foo::bar().
* 如果函数将self, &self, 或&mut self作为它的第一个参数，
* 它也可以使用方法调用语法来调用，这是任何面向对象的程序员都熟悉的功能，例如foo.bar().
*/
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

//  -------关联函数-------
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}
fn main() {
    // ----------------定义方法-------------------
    // 定义于Rectangle结构体的area方法

    println!("Hello, world!");
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 20,
        height: 20,
    };
    println!("矩形的面积{}", rect1.area());
    println!("矩形的面积{}", rect1.can_hold(&rect2))
}
