### Rust采集环境变量

```rust
 use std::env
 fn main(){
    let args = Vec<String> = env::args().collect() ; //采集环境变量
    println!(":?",args);
 }
```

### 读取文件和命令行处理

``` rust
use std::env;
use std::fs;
 fn main(){
    let args = Vec<String> = env::args().collect();
    let query = &args[1];
    let filename = &args[2];
    println!("Searching for {}",query);
    println!("In file {}",filename); // 读取环境变量

    //  读取文件
    let contents = fs::read_to_string(filename).expect("Somthing went wrong reading the file");
    println!("With text:\n {}",contents)
 }
   
 fn parse_config1(args:&[String]) -> (&str,&str){
    let query = &args[1];
    let filename = &args[2];
    (query,filename)`
 }

```
