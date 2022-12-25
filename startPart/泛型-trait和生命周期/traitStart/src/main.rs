// pub trait Summary {
//     fn summarize(&self) -> String {
//         String::from("(Read more...)")
//     }
// }
// // 定义结构体 1
// pub struct NewArticle {
//     pub headling: String,
//     pub location: String,
//     pub author: String,
//     pub content: String,
// }

// // 嫁接结构体上的接口
// impl Summary for NewArticle {}
// // 新建结构体
// pub struct Tweet {
//     pub username: String,
//     pub content: String,
//     pub reply: bool,
//     pub retweet: bool,
// }
// // 嫁接结构体上的接口
// impl Summary for Tweet {
//     fn summarize(&self) -> String {
//         format!("{}:{}", self.username, self.content)
//     }
// }

// fn main() {
//     println!("Hello world");
//     let article = NewArticle {
//         headling: String::from("horse_ebook"),
//         location: String::from("of course,as you probably already know, people"),
//         author: String::from("of course,as you probably already know, people"),
//         content: String::from("of course,as you probably already know, people"),
//     };
//     println!("1 new is tweet :{}", article.summarize());
//     let tweet = Tweet {
//         username: String::from("horse_ebook"),
//         content: String::from("of course,as you probably already know, people"),
//         reply: false,
//         retweet: false,
//     };
//     println!("2 new is tweet :{}", tweet.summarize());
// }


// ------- 第二部分trait当作函数参数或者返回值 -------
fn main(){}