fn main() {
    println!("所有权与函数");
    // 定义：将值传递给函数在语义上与给变量赋值相似，向函数传递值可能会移动或者复制，就像赋值语句一样。
    let s1: String = gives_ownership(); // gives_ownership 将返回值
    let s2: String = String::from("Hello"); // s2 进入作用域
    let s4: String = takes_and_gives_back(s2); // s2 被移动到 takes_and_gives_back 函数中，他将返回值移给s4
    // let s5 = s2; // 报错 error: use of moved value: `s2` 
    // 因为 s2 的所有权已经移到给takes_and_gives_back函数中。
    println!("------------------------------------------------");

    println!("返回值与作用域");
    // 返回值也可以转移所有权。
}

fn gives_ownership() -> String {
    let s3: String = String::from("Hello s1");
    s3
}
fn takes_and_gives_back(a_string: String) -> String {
    a_string
}


