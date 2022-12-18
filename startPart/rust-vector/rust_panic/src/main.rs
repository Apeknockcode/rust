// fn main() {
//     // println!("Hello, world!");
//     // -------错误处理-------
//     /*
//      * Rust 将错误组合成两个主要类别：“可恢复错误”和“不可恢复错误”，
//      * “可恢复错误”通常代表向用户报告错误和重试操作是合理的情况，，比如:"未找到文件"；
//      * “不可恢复操作”通常是BUG的同义词。
//      * */
//     println!("-------Rust中的错误处理：panic!-------");
//     // panic!("crash and burn");
//     /*
//      * 例如：
//      * let v = [1,2,3,4];
//      * v[999]; //出错
//      * //或者
//      * use::std::fs::File;
//      * let f = File::open("Hello.txt")// 出错：找不到文件
//      * */
// }

// use std::fs::File;
// fn main() {
//     let f = File::open("hello.txt");
//     let f = match f {
//         Ok(file) => file,
//         Err(error) => {
//             panic!("文件不存在")
//         }
//     };
// }

// fn main() {
//     use std::io::ErrorKind;
//     let f = File::open("hello.txt");
//     let f = match f {
//         Ok(file) => file,
//         Err(error) => match error.kind() {
//             // 文件不存在
//             ErrorKind::NotFound => {
//                 // 创建文件
//                 match File::create("hello.txt") {
//                     Ok(success) => success,
//                     Err(fail) =>  panic!("Tried to create file but there was a problem:{:?}",fail),
//                 }
//             }
//             other_error => panic!("There was a problem opening the file : {:?}", other_error),
//         },
//     };
// }

//  rust -  painc - result
use std::fs;
use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;
fn main() {
    let username = read_username_from_file2().expect("unable to get username");
    println!("读取文件里面的内容：{}", username);
}
fn read_username_from_file2() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

fn read_username_from_file1() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");
    // 打开失败的两种情况
    let mut f = match f {
        Ok(file) => file,
        Err(e) => match e.kind() {
            // 文件不存在的情况
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(error) => panic!("Tried to create file but there was a problem:{:?}", error),
            },
            other_error => panic!("There was a problem opening the file : {:?}", other_error),
        },
    };
    let mut s = String::new();
    // 文本读入字符串
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
