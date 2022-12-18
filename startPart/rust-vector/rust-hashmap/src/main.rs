// 使用系统提供的标准库
use std::collections::HashMap;
fn main() {
    println!("rust hashmap");
    /*
     * 方便我们快速查找，
     * HashMap<K,V> 类型储存一个键类型K对应一个值类型V的映射，他通过一个哈希函数来实现映射，
     * 决定如何将键和值放入内存中。hashmap 可以用于需要类型作为键来寻找数据等情况，而不是像vector那样通过索引。
     * */
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let inital_scores = vec![10, 50];
    // 基于vec数组创造hashmap；
    let scores_other: HashMap<_, _> = teams.iter().zip(inital_scores.iter()).collect();
    println!("基于vec数组创造hashmap:{:?}", scores_other);
    // -------snip-------
    println!("哈希map 和 所有权");
    /*
     * 对于像i32这样的实现了Copy trait 的类型，其值可以拷贝进哈希map，对于像string 这样拥有所有权的值，其值将被
     * 移动而哈希map会成为这些值的所有者，
     * */
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // println!("{}", field_name);// 报错。所有权已经被转移了。
    // -------snip-------
    println!("访问哈希map中的值");
    let mut scores1 = HashMap::new();
    scores1.insert(String::from("Blue"), 10);
    scores1.insert(String::from("Yellow"), 50);
    scores1.insert(String::from("Yellow"), 5);
    let team_name = String::from("Blue");
    let scores2 = scores1.get(&team_name);
    // println!("{}",*scores2); //报错：type `Option<&{integer}>` cannot be dereferenced -> 不能取消引用类型`Option<&{integer}>`
    // 修正
    println!("{}", *scores2.unwrap());
    println!("{:?}", scores1);
    // 循环scores1
    for (k, i) in scores1.iter() {
        println!("键：{:?}，值：{:?}", k, i)
    }
    // for (k, i) in scores1 {
    //     println!("键：{:?}，值：{:?}", k, i)
    // }
    // -------snip-------
    println!("-------只在键没有对应值时插入-------");
    /*
     * 我们经常会检查某一个特定的键是否有值，如果没有就插入一个值，为此哈希map有一个特殊的API ，叫做entry
     * 它获取我们想要的检查的键作为参数，entry函数的返回值是一个枚举,entry代表了可能不存在的值，
     * 比如我们想要检查黄队的键是否关联了一个值，如果没有，就插入值 5，对于蓝队也是如此。
     */
    scores1.insert(String::from("Blue"), 10);
    scores1.entry(String::from("Yellow")).or_insert(5);
    scores1.entry(String::from("Blue")).or_insert(5);
    println!("{:?}", scores1);
    println!("-------根据旧的值来更新它-------");
    let text = "Hello world wonderful word";
    let mut map = HashMap::new();
    println!("{:#?}", text.split_whitespace());
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
