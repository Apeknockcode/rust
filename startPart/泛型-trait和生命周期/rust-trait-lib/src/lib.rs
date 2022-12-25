#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4)
    }
}

pub trait Summary {
    fn summary(&self) -> String; // 类似接口的含义
}

// 定义结构体 1
pub struct NewArticle {
    pub headling: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// 嫁接在结构体上的接口
impl Summary for NewArticle {
    fn summary(&self) -> String {
        format!("{},by {} ({})", self.headling, self.author, self.location)
    } // 类似接口的含义
}
// 结构体 2
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

// 嫁接在结构体上的接口
impl Summary for Tweet {
    fn summary(&self) -> String {
        format!("{},by {}", self.username, self.content)
    } // 类似接口的含义
}

// 调用
pub fn Run() {
    let tweet = Tweet {
        username: String::from("horse_ebook"),
        content: String::from("of course,as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new is tweet :{}", tweet.summary());
}
