struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
fn main() {
    println!("结构体的知识点");
    let mut user1 = User {
        username: String::from("someusername123"),
        email: String::from("someusername@qq.com"),
        sign_in_count: 1,
        active: true,
    };
    let mut user2 = User {
        username: String::from("someusername123"),
        email: String::from("someusername@qq.com"),
        sign_in_count: user1.sign_in_count,
        active: user1.active,
    };
    let mut user2 = User {
        username: String::from("someusername123"),
        email: String::from("someusername@qq.com"),
        ..user1
    };
    user1.email = String::from("someusername@163.com");
    // ----------------------使用没有命名字段的元组结构体来创建不同的类型---------------------------------
    /*
     * 要定义元组结构体，以struct关键字和结构体名开头并后跟元组中的类型。
     * 例如：
     *  struct Color(i32,i32,i32)
     *  let black = Color(0,0,0);
     * */
    struct Color(i32, i32, i32);
    let black = Color(0, 0, 0);
    // ----------------------没有任何字段的类单元结构体---------------------------------
    /*
     * 定义一个没有任何字段的结构体，被称为 类单元结构体，因为它们类似于（） ，即unit类型，
     * 类单元结构体常常在你想要在某个类型上实现trait但不需要在类型中储存数据的时候发挥作用。
     * */
    // ----------------------结构体数据的所有权---------------------------------
    let width: i32 = 30;
    let height: i32 = 50;
    println!("矩形的面积是,{}*{}={}", width, height, width * height);
    let react: (u32, u32) = (30, 50);
    println!("矩形的面积是,{}", area(react));
    // 定义结构体
    let rectangle = Rectangle{
        width:30,
        height:50,
    };
    println!("矩形的面积是,{}", area1(&rectangle));
    // ----------------------通过派生trait增加实用功能---------------------------------
    println!("打印结构体,{:#?}",rectangle)//打印结构体
}
#[derive(Debug)] // 方便打印输出信息
struct Rectangle {
    width: u32,
    height: u32,
}
// 计算面积的函数-通过结构体
fn area1(dimensions: &Rectangle) -> u32 
{
    dimensions.width * dimensions.height
}
// 计算面积的函数-通过元组来计算
fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
// 创建一个函数 构造一个结构体
fn build_user(email: String, username: String) -> User {
    User {
        username: username,
        email: email,
        sign_in_count: 1,
        active: true,
    }
}
// 或者
fn build_userx(email: String, username: String) -> User {
    User {
        username,
        email,
        sign_in_count: 1,
        active: true,
    }
}
