// fn main() {
//     // --------------match 控制流运算符 --------------
//     /*
//      * 它允许我们将一个值与一系列的模式相比较，并根据相匹配的模式执行相应代码，
//      * 模式可以由字面值，变量，通配符和许多其他内容构成。
//      * */
//      enum Coin {
//         Penny,
//         Nicker,
//         Dime,
//         Quarte,
//     }
//     fn value_in_cents(coin: Coin) -> u8 {
//         // 根据不同的类型处理
//         match coin {
//             Coin::Penny => {
//                 println!("Lucky Penny");
//                 1
//             }
//             Coin::Nicker => 5,
//             Coin::Dime => 10,
//             Coin::Quarte => 25,
//         }
//     }
// }

// #[derive(Debug)]
// enum UsState {
//     Alabama,
//     Alaska,
// }
// enum Coin {
//     Penny,
//     Nicker,
//     Dime,
//     Quarte(UsState),
// }
// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => {
//             println!("Lucky Penny");
//             1
//         }
//         Coin::Nicker => 5,
//         Coin::Dime => 10,
//         Coin::Quarte(state) =>{
//             println!("State Quarte from {:?}",state);
//             25
//         },
//     }
// }
// fn main() {
//     value_in_cents(Coin::Quarte(UsState::Alaska));
// }
// fn main() {
//     fn plus_one(x: Option<i32>) -> Option<i32> {
//         match x {
//             // 匹配类型
//             Some(i) => Some(i + 1),
//             None => None,
//         }
//     }
//     let five = Some(5);
//     let six: Option<i32> = plus_one(five);
//     let none: Option<i32> = plus_one(None);
// }

fn main() {
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }
}
