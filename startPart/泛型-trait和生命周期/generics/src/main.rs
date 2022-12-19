// fn main() {
//     println!("Hello,范型!");
//     /*
//      * 范型是具有类型或者其他属性的抽象替代.
//      * 可以表达泛型的属性，比如他们的行为或者其他泛型相关联，而不需要在编写和编译代码时知道他们在这里实际上代表什么。
//      * 同理为了编写一份可以用于多种具体值的代码，函数并不知道其参数为何值，这时就可以让函数获取泛型而不是像i32 或者 string 这具体值，
//      *
//      * 之后，我们讨论trait ，这是一个定义泛型行为的方法，trait可以与泛型结合来将泛型限制为拥有特定行为的类型，而不是任意类型。
//      * 最后介绍生命周期。他是一类允许我们向编译器提供引用如何相互关联的泛型，Rust的生命周期功能允许在很多场景下借用的同时仍然使用编译器能够检查这些引用的有效性。
//      *
//      * */
//     println!("提取函数来减少重复");
//     /*
//      *  不使用泛型的处理重复的技术，
//      *  提取一个函数，当熟悉这个技术后，我们将使用相同的机制获取一个泛型函数，如同你识别出可以提取到函数中重复的代码那样，你也可以识别出能够使用泛型的重复代码
//      * */
//     //  例如：
//     let number_list = vec![34, 50, 35, 100, 65];
//     let result = largest(&number_list);
//     // let mut largest = number_list[0];
//     // for number in number_list {
//     //     if number > largest {
//     //         largest = number;
//     //     }
//     // }
//     println!("The largest number is : {}", result);
//     assert_eq!(result, 100); // 测试，结果与预期是否一致。
//     let number_list = vec![102, 34, 6000, 89, 54, 3, 43, 8];
//     // let mut largest = number_list[0];
//     // for number in number_list {
//     //     if number > largest {
//     //         largest = number;
//     //     }
//     // }
//     // 改造
//     let result = largest(&number_list);
//     println!("The largest number is : {}", result);
//     assert_eq!(result, 6000); // 测试，结果与预期是否一致。
// }

// fn largest(list: &[i32]) -> i32 {
//     // 指定list的类型i32 返回是一个i32
//     let mut largest = list[0];
//     for &number in list {
//         if number > largest {
//             largest = number;
//         }
//     }
//     largest // 返回值
// }

// -------- 在函数定义中使用泛型 --------
// use std::cmp::PartialOrd;
// fn main() {
//     let number_list = vec![34, 50, 35, 100, 65];
//     let result = largestT(&number_list);
//     println!("The largest number is : {}", result);
//     let number_list = vec![102, 34, 6000, 89, 54, 3, 43, 8];
//     let result = largestT(&number_list);
//     println!("The largest number is : {}", result);
// }

// fn largestT<T: PartialOrd>(list: &[T]) -> &T {
//     let mut largest = &list[0];
//     for item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }

// -------- 结构体定义中的泛型 --------
/*
 * 同样也可以用 <> 语法来定义结构体，它包含一个或者多个泛型参数类型字段。
 *
 * */

// struct Point<T, U> {
//     x: T,
//     y: U,
// }
// fn main() {
//     // 泛型
//     let integer = Point { x: 5, y: 10 };
//     let float = Point { x: 1.0, y: 2.0 };
//     let integer_float = Point { x: 2, y: 1.0 };
// }

// struct Point<T> {
//     x: T,
//     y: T,
// }
// impl<T> Point<T> {
//     fn x(&self) -> &T {
//         &self.x
//     }
// }
// fn main() {
//     // 泛型
//     // let integer = Point { x: 5, y: 1.0 };//当前类型不匹配
//     let p = Point { x: 1, y: 2 };
//     println!("{}", p.x())
// }

// -------- 枚举中定义泛型 --------
enum Result<T, U> {
    OK(T),
    Err(U),
}

fn main(){
    
}
