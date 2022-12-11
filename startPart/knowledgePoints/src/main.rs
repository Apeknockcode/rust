use std::mem;
// 常量 和 变量
// 变量的可变性
fn main() {
    println!("part1 ： 变量的可变性");
    // const x:i32 = 5; // let 常量
    // println!("x is {}", x); // 输出 x is 5;
    // x = 6; // 报错  cannot assign twice to immutable variable
    // println!("x is {}", x);

    // 修正 - 更改 x 的数值
    let mut x: i32 = 5;
    println!("x is {}", x); // 输出 x is 5
    x = 6;
    println!("new x is {}", x); // 输出 new x is 6
                                // 概念
                                /*
                                 * 常量和变量的区别:
                                 * 常量类似于不可变量，常量是绑定到一个名称的不允许改变的值。不过常量与比变量还是有一些区别的。
                                 * 首先，不允许对常量使用mut
                                 * 声明常量使用const 关键字而不是let， 并且必须注明值的类型。
                                 * 常量可以在任何作用域声明，包括全局作用域;
                                 * 常量只能被设置为常量表达式，不能是函数调用的结果，或者任何其他只能在运行时计算出的值.
                                 */

    println!("part2 ：变量的隐藏");
    let y: i32 = 12;
    let y = y + 2;
    println!("y 的最新值 ：{}", y); // 输出 ： y 的最新值 ：14
    let y = y - 3;
    println!("y 的最新值 ：{}", y); // 输出 ： y 的最新值 ：11

    let mut spaces = "  ";
    spaces = "123"; // 不可以改变类型
    println!("spaces is {}", spaces);

    println!("part3 : 数据类型");
    println!("元组的获取");
    let z = (1.2, 2.3, 3.3);
    // let (zz,zzz,zzzz) = z;
    let z_one = z.0;
    let z_two = z.1;
    let z_three = z.2;
    println!("The Value is zOne of {}", z_one);
    println!("The Value is zTwo of {}", z_two);
    println!("The Value is zThree of {}", z_three);
    // let z = (500,6.3,1);
    // let five_hundred= z.0;
    // let six_point_four= z.1;
    // let one= z.2;
    // println!("The Value of five_hundred is:{}",five_hundred );
    // println!("The Value of six_point_four is:{}",six_point_four );
    // println!("The Value of one is:{}",one );

    println!("数组类型");
    let array = [1, 2, 3, 4, 5, 6];
    let first = array[0];
    println!("first : {}", first);
    let other_array = [3; 4]; //  实则是 let other_array =[3,3,3,3]
    let get_other_array_0 = other_array[0]; //
    let get_other_array_1 = other_array[1]; //
    let get_other_array_2 = other_array[2]; //
    let get_other_array_3 = other_array[3]; //
                                            // let get_other_array_4 = other_array[4]; // 报错 index out of bounds: the length is 4 but the index is 4
                                            // println!("The Value of other_array is {}", other_array); // 报错：cannot be formatted with the default formatter
    println!("The Value of other_array is {}", get_other_array_0); //输出 3
    println!("The Value of other_array is {}", get_other_array_1); //输出 3
    println!("The Value of other_array is {}", get_other_array_2); //输出 3
    println!("The Value of other_array is {}", get_other_array_3); //输出 3
                                                                   // println!("The Value of other_array is {}", get_other_array_4);

    println!("数组的切片");
    // 定长数组（类型标记是多余的）
    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    // 所有元素可以初始化成相同的值
    let ys: [i32; 500] = [0; 500];

    // 下标从 0 开始
    println!("first xs of the array : {}", xs[0]);
    println!("first ys of the array : {}", ys[0]);

    // 'len' 返回数组的长度
    println!("xs is array size: {}", xs.len());
    println!("ys is array size: {}", ys.len());

    // 数组是在栈中分配的
    println!("array occupies {} bytes", mem::size_of_val(&xs));

    // 数组可以自动被借用成为 slice
    println!("borrow the whole  array as a slice");

    analyze_slice(&xs[1..2]); // 切片 取得 1-2 之间

    println!("part5 函数");
    let a = 5;
    let b = {
        let a = 3;
        // a + 1; // 注意 加;是一个语句
        a + 1 // 不加;是一个返回值
    };
    println!("The value of b is : {}", b);
}
// :&[i32]
fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} element", slice.len());
}
