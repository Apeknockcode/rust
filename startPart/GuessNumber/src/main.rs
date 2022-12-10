use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main1() {
    println!("猜数字");
    println!("请输入数字");
    let mut guess = String::new(); // 创建变量开辟内存。
    io::stdin().read_line(&mut guess).expect("读取失败"); //读取键盘输入
    println!("你猜的数字：{}", guess);
}

// 生成随机数
fn main2() {
    println!("猜数字");
    println!("请输入数字");
    let right_num = rand::thread_rng().gen_range(1..101);
    let mut guess = String::new(); // 创建变量开辟内存。
    io::stdin().read_line(&mut guess).expect("读取失败"); //读取键盘输入
    println!("你猜的数字：{}", guess);
    println!("正确的数字：{}", right_num);
}

//
fn main3() {
    println!("猜数字");
    println!("请输入数字");
    let right_num = rand::thread_rng().gen_range(1..101);
    let mut guess = String::new(); // 创建变量开辟内存。
    io::stdin().read_line(&mut guess).expect("读取失败"); //读取键盘输入
    println!("你猜的数字：{}", guess);
    let guess: u32 = guess.trim().parse().expect("转换失败"); // 类型转化
                                                              // 判断用户猜的数字 大于还是小于
    match guess.cmp(&right_num) {
        Ordering::Less => {
            println!("猜小了")
        }
        Ordering::Greater => {
            println!("猜大了")
        }
        Ordering::Equal => {
            println!("猜中了");
        }
    }
    println!("正确的数字：{}", right_num);
}

// 循环 直到用户猜到
fn main() {
    let right_num = rand::thread_rng().gen_range(1..101);
    println!("猜数字游戏");
    loop {
        // 循环的作用
        println!("请输入数字");
        let mut guess = String::new(); // 创建变量开辟内存。
        io::stdin().read_line(&mut guess).expect("读取失败"); //读取键盘输入
        println!("你猜的数字：{}", guess);
        let guess: u32 = guess.trim().parse().expect("转换失败"); // 类型转化
                                                                  // 判断用户猜的数字 大于还是小于
        match guess.cmp(&right_num) {
            Ordering::Less => {
                println!("猜小了")
            }
            Ordering::Greater => {
                println!("猜大了")
            }
            Ordering::Equal => {
                // 跳出循环
                println!("猜中了");
                println!("正确的数字：{}", right_num);
                break;
            }
        }
    }
}
