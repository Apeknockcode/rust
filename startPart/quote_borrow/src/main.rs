// fn main() {
//     println!("引用和借用");
//     let s1: String = String::from("hello");
//     let len = calculate_length(&s1);
//     println!("len is size: {}", len);
//     println!("s1 的所有权是否转移,输出s1 {}", s1); // s1的所有权没转移
// }
// fn calculate_length(s_length: &String) -> usize {
//     s_length.len()
// }
// fn main() {
//     println!("可变引用");
//     // 限制🚫：在特定作用域中的特定数据只能有一个可变引用。
//     let mut s1: String = String::from("Hello");
//     change(&mut s1);
//     // println!("change 返回值是：{}", );
// }
// fn change(s: &mut String) {
//     s.push_str(", rust");
//     println!("s 的值是 {}", s);
// }

fn main() {
    println!("悬垂引用");
    let reference_to_nothing = dangle();
}
fn dangle() -> String {
    // dangle 返回一个字符串的引用；
    let s = String::from("Hello");// 创建一个s的内存是一个新的字符串
    s
    // 返回字符串s的引用
}// 这里 s 离开作用域并被丢弃，其内存被释放， 危险⚠️
/*
 * 因为s 在dangle函数内创建的，当dangle的代码执行完毕后，s 将被释放，
 * 不过我们尝试返回它的引用，这意味着这个引用会指向一个无效的String 这是不对的。
 * Rust 不允许我们这么做/
 * 解决办法：
 * 直接返回String
 * 
*/
