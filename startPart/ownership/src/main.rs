// rust 所有权
fn main() {
    // let mut s: String = String::from("Hello"); // 开辟内存
    // s.push_str(", rust!");
    // println!("s is {}", s);
    {
        let mut s: String = String::from("Hello"); // 开辟内存
        s.push_str(", rust!");
        println!("s is {}", s);
    }
    //  println!("s is {}", s);// error: cannot find value `s` in this scope; label: not found in this scope
    println!("只在栈上的数据：拷贝");
    /**
     * 什么类型是copy的呢？
     * 任何简单标量值的组合可以是copy的，不需要分配内存或者某种形式资源的类型是copy的。如下是一些Copy的类型：
     * 所有的整数类型。比如 ： u32；
     * 布尔类型，bool，它的值是 true 和 false;
     * 所有的浮点类型，比如：f64；
     * 字符串类型 ：char ，
     * 元组，当且仅当其包含的类型也都是Copy 类型的时候，比如：（i32,i32） 是copy 的。但是（i32，string） 就不是
     * */
    let x = 5;
    let y = x;
    if x == y {
        println!("x和y相等",);
    } else {
        println!("x和y不相等",);
    }
    println!("x的值等于{},y的值等于{},", x, y);

    // let str = String::from("Hello"); // 用到的内存是相同的。
    // let str1 = str; //报错：value borrowed here after move （翻译：移动后在此借用的值）
    // if str == str1 {
    //     println!("x和y相等", );
    // }else{
    //     println!("x和y不相等", );
    // }
    // println!("str的值等于{},str1的值等于{},", str, str1);
    println!("------------------------------------------------------");
    println!("变量与数据交互方式（二） ： 克隆");
    let s1 = String::from("hello");
    let s2 = s1.clone();
    if s1 == s2 {
        println!("x和y相等",);
    } else {
        println!("x和y不相等",);
    }
    println!("s1的值等于{},s2的值等于{},", s1, s2);
    println!("------------------------------------------------------");
}
