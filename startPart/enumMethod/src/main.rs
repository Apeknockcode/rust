// --------Rust 中的枚举与模式匹配 初级--------
// fn main() {
//     #[derive(Debug)]
//     enum IpAddressKind {
//         v4,
//         v6,
//     }
//     #[derive(Debug)]
//     struct IpAddr {
//         kind: IpAddressKind,
//         address: String,
//     }

//     let homeip = IpAddr {
//         kind: IpAddressKind::v4, //枚举体
//         address: String::from("127.0.0.1"),
//     };
//     let workip = IpAddr {
//         kind: IpAddressKind::v6, //枚举体
//         address: String::from("192.168.0.57"),
//     };
//     println!("homeip is {:#?}", homeip)
// }

// fn main() {
//     #[derive(Debug)]
//     enum Message {
//         Quit,
//         Move { x: i32, y: i32 },
//         Wirte(String),
//         ChangeColor(i32, i32, i32),
//     }
//     impl Message {
//         fn call(&self) {
//             //
//             println!("wirte is {:#?}", self);
//         }
//     }
//     let m = Message::Wirte(String::from("Hello"));
//     m.call()
// }

// fn main() {
//     enum IpAddr {
//         v4(String),
//         v6(String),
//     }
//     let home = IpAddr::v4(String::from("127.0.0.1"));
//     let loopback = IpAddr::v6(String::from("192.168.1.57"));
// }
// fn main() {
//     #[derive(Debug)]
//     enum IpAddr {
//         v4(u8,u8,u8,u8),
//         v6(String),
//     }
//     let home = IpAddr::v4(127,0,0,1);
//     let loopback = IpAddr::v6(String::from("::1"));
//     println!("loopback is {:#?}",loopback)
// }

// fn main() {
//     // enum Option<T> {
//     //     Some(T),
//     //     None,
//     // }
//     let some_number = Some(5);
//     let some_string = Some("a string");
//     let absent_number: Option<i32> = None;
// }

fn main(){
    
}
