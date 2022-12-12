// fn main(){
//     let number :i32 =3;
//     if number <10{
//         println!("number 小于 10" );
//     }else{
//         println!("number 大于 10");
//     }
// }
// -------------------------------------------------------------------
// use std::io;
// fn main() {
//     println!("请输入你的分数");

//     let mut score = String::new();
//     io::stdin().read_line(&mut score).expect("读取失败");
//     let score: i32 = score.trim().parse().expect("转化失败");

//     if score > 90 && score < 100 {
//         println!("优秀");
//     } else if score > 80 && score < 90 {
//         println!("良好");
//     } else if score > 70 && score < 80 {
//         println!("合格");
//     } else if score > 60 && score < 70 {
//         println!("凑合");
//     } else {
//         println!("垃圾");
//     }
// }

// fn main() {
//     let condition: bool = false;
//     let number: i32 = if condition { 5 } else { 6 };
//     // let otherValue:i32 = if condition { 5 } else { "six" }; //报错 确保类型统一
//     if condition {
//         println!("当 condition 为 false 时,the number value is {}", number);
//     } else {
//         println!("当 condition 为 true 时,the number value is {}", number);
//     };
// }

// 死循环  和 跳出循环
// fn main() {
//     let mut counter: i32 = 0;
//     // 死循环
//     let result: i32 = loop {
//         counter += 1;
//         if counter == 10 {
//             break  // 跳出循环 执行本次动作
//             counter * 2;
//         }
//     };
//     println!("The result is {}", result);
// }

// for循环 和 while循环
// use std::mem;
fn main() {
    let mut number: i32 = 12;
    // while循环
    while number != 0 {
        println!("{}!", number);
        number = number - 1;
    }
    println!("LIFT OFF!");

    // while循环
    let a: [i32; 5] = [10, 20, 30, 40, 50];
    let mut index: usize = 0;

    // let mut a_length:usize as i32 = a.len();
    // println!("array occupies {} bytes", mem::size_of_val(&a));
    // let mut len_size =a_length.parse::<i32>().unwrap().expect("转化失败");
    // println!(" a is size: {}", a.len());
    while index < a.len() {
        println!("the value is : {}", a[index]);
        index = index + 1
    }

    // for 循环
    let mut iter = a.iter();
    println!("a.iter() is output {:?}", iter);
    // 1
    for element in a.iter() {
        println!("1.the value is {}", element)
    }
    // 2
    for element in a.iter().rev(){
        println!("2.the value is {}", element)
    }


    // 所有权
    /*
     * 所有权的规则
     * 1. Rust中每一个值都有一个被称为其所有权的变量
     * 2. 值在任一时刻有且只有一个所有者。
     * 3. 当所有者（变量）离开变量作用域，这个值将被丢弃。
     * */ 
     // s 在这里是无效的。它尚未声明
     let s = "Hello";// 此处的s 是有销的
     // 使用s
}
// 此时 s 的作用域已经结束了，s 不再有效。
