fn main1() {
    //  函数的泛生命周期
    let string1 = String::from("abcd"); // 字符串
    let string2 = "efghijkdhasudhaishduiad"; // 字符串地址
    let result = longest(string1.as_str(), string2); //获取最长的字符串
    println!("The longest string is {}", result);
}
// 取得字符串最长的

/*
fn longest(str1: &str, str2: &str) -> &str {
    if str1.len() > str2.len() {
        str1
    } else {
        str2
    }
}
报错：
error[E0106]: missing lifetime specifier
 --> src/main.rs:9:39
  |
9 | fn longest(str1: &str, str2: &str) -> &str {
  |                  ----        ----     ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `str1` or `str2`
help: consider introducing a named lifetime parameter
  |
9 | fn longest<'a>(str1: &'a str, str2: &'a str) -> &'a str {
  |           ++++        ++             ++          ++

error[E0425]: cannot find value `String1` in this scope
 --> src/main.rs:5:26
  |
5 |     let result = longest(String1.as_str(), string2); //获取最长的字符串
  |
*/

// 解决办法 引用 ‘a 处理生命周期问题
fn longest<'a>(str1: &'a str, str2: &'a str) -> &'a str {
    if str1.len() > str2.len() {
        str1
    } else {
        str2
    }
}

// 另一个例子： 成立
fn main() {
    //  函数的泛生命周期
    let string1 = String::from("abcd"); // 字符串
    let result;
    let string2 = "efghijkdhasudhaishduiad"; // 字符串地址
    result = longestA(string1.as_str(), string2); //获取最长的字符串
    println!("The longest string is {}", result);
}
fn longestA<'a>(str1: &'a str, str2: &'a str) -> &'a str {
    if str1.len() > str2.len() {
        str1
    } else {
        str2
    }
}
