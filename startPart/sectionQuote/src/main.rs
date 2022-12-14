fn main1() {
    println!("Hello, world!");
    let mut s: String = String::from("Hello world");
    let word: usize = first_word(&s);
    s.clear(); //这清空字符串，使其等于 “”
    println!("word is {}", word);
    // word 在此处的值仍然是 5
    // 但是没有更多的字符串让我们可以有效的应用数值 5 。word 的值选在完全无效！
}
fn first_word(s: &String) -> usize {
    let bytes: &[u8] = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn main() {
    let s: String = String::from("Hello world");
    let hello: &str = &s[0..5]; // 切片 0-5
    let world: &str = &s[6..11];
    println!("hello is value of {}", hello);
    println!("world is value of {}", world);
    // s.clear();// 报错÷
    // 因为rust 中当我们借用一个变量的时候，我们不能去修改。
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let slice: &[i32] = &a[1..3];
}
