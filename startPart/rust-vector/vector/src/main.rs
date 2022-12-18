// fn main() {
//     let mut v1: Vec<&str> = Vec::new();
//     let mut v3: Vec<i32> = Vec::new();
//     v3.push(31);
//     v3.push(32);
//     v3.push(33);
//     v3.push(34);
//     v3.push(35);
//     {
//         let v2 = vec![1, 2, 3, 4, 5];
//     } // 内存被回收
//     let num3: &i32 = &v3[2];
//     println!("num3 is {}", num3);
//     match v3.get(1) {
//         Some(num3) => println!("num3 存在 是 {}", num3)  ,
//         _ => println!("num3 不存在"),
//     }
// }

//  字符串的加法
// fn main() {
//     let s1 = String::from("Hello");
//     // let h = s1[0];
//     let s2 = String::from("World");
//     let s3 = s1 + &s2;
//     println!("{}", s2);
//     println!("{}", s3);
// }

// // 字符串的加法
// fn main() {
//     let s1 = String::from("tic");
//     let s2 = String::from("tac");
//     let s3 = String::from("toe");
//     let s = s1 + "-" + &s2 + "-" + &s3;
//     println!("{}", s);
// }
// // 字符串的加法
// fn main() {
//     let s1 = String::from("tic");
//     let s2 = String::from("tac");
//     let s3 = String::from("toe");
//     let s = format!("{}-{}-{}", s1, s2, s3); //格式化
//     // println!("{}-{}-{}", s1, s2, s3);
//     println!("{}", s);
// }

// 索引字符串
// fn main() {
//     let s = String::from("hello");
//     // let h = s[0]; // 报错：// Rust 的字符串不支持索引，
//     let len = String::from("hola").len();
//     //  内部表现
//     /*
//     * String 是一个Vec<u8>的封装，
//     * let len = String::from("hola").len();
//     * 在这里，len的值是 4 ，这意味着字符串中 “hola”的Vec的长度是四个字节，这里每一个字母的UTF-8编码都占用一个字节，
//     * 而韩文，中文，汉字，日文等占用3个字节，英文占用一个字节。
//     */
//     let  hello = "你好,Rust";
//     // let  s1= &hello[0]; // 报错
//     // 修正
//     let s2 = &hello[0..3];
//     println!("s slice value of {}",s)
// }

// 遍历字符串
fn main() {
    let s = "그때옆으로3123을보고";
    for i in s.chars() {
        // 遍历每一个字符
        println!("遍历值：{}", i);
    }
    println!("---------------snip--------------------");
    for item in s.bytes() {
        //  遍历每一个字节/
        println!("遍历值：{}", item);
    }
}
