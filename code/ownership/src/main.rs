// fn main() {
//     {
//         // s 在这里无效
//         let s = "hello"; // 从这里开始，s 是有效的
//                          // 使用 s
//                          // ...
//     } // 此作用域已结束，s 不再有效

//     // let mut s = String::from("hello");
//     // s.push_str(", world!"); // push_str() 在字符串后追加字面值
//     // println!("{}", s); // 打印 hello, world!

//     {
//         let s = String::from("hello"); // 从此处起，s 是有效的
//                                        // 使用 s
//                                        // ...
//     } // 此作用域已经结束，s 不再有效

//     {
//         let x = 5;
//         let y = x;
//         println!("{}", y);
//     }

//     {
//         let s1 = String::from("hello");
//         let s2 = s1;
//         // println!("s1 is {}, s2 is {}", s1, s2); // 会报错，此时 s2 对于 s1 只是引用了 s1 所指向的堆上的内容，当 String 类型的数据如果只是被引用后，String 上的其他引用就会失效，只有当前指向 String 的引用才是有效的，所以此时会报错。
//     }

//     {
//         let s1 = String::from("hello");
//         let s2 = s1.clone();
//         println!("s1 = {}, s2 = {}", s1, s2); // 此时属于将 s1 的内容克隆到 s2 身上了，所以不会报错
//     }

//     {
//         let x = 5;
//         let y = x;
//         println!("x = {}, y = {}", x, y); // 只在栈上的数据，直接就是拷贝
//     }
// }

// 理解所有权与函数
// fn main() {
//     let s = String::from("hello"); // s 进入作用域
//     takes_ownership(s); // s 的值移动到函数里
//                         // 所以这里 s 不再有效
//     let x = 5; // x 进入作用域

//     makes_copy(x); // x 应该移动到函数里，但 i32 是 Copy 的，所以在后面可继续使用 X
// } // 这里 x 先移出了作用域，然后是 s，因为 x 的值已被移走
//   // 其他没有特别之处

// fn takes_ownership(some_string: String) {
//     // some_string 进入作用域
//     println!("{}", some_string);
// } // 这里 some_string 移出作用域并调用 `drop` 方法
//   // 占用的内存被释放

// fn makes_copy(some_integer: i32) {
//     // some_integer 进入作用域
//     println!("{}", some_integer)
// } // 这里，some_integer 移出作用域。其余相同

// 返回值也可以转移所有权
// fn main() {
//     let s1 = gives_ownership(); // gives_ownership 将返回值转移给 s1
//     let s2 = String::from("hello"); // s2 进入作用域
//     let s3 = takes_and_gives_back(s2); // s2 被移动到 takes_and_gives_back 中，它也将返回值移给 s3
// } // 这里，s3 移出作用域并被丢弃，s2 也移出作用域，但已被移走，所以什么也不会发生，s1 离开作用域并被丢弃

// // gives_ownership 会将返回值移动给调用它的函数
// fn gives_ownership() -> String {
//     let some_string = String::from("yours"); // some_string 进入作用域
//     some_string // 返回 some_string，并移出给调用的函数
// }

// // takes_and_gives_back 将传入字符串并返回该值
// fn takes_and_gives_back(a_string: String) -> String {
//     // a_string 进入作用域
//     a_string // 返回 a_string 并移出给调用的函数
// }

// 可以利用元组将返回多个值（包括所有权）
fn main() {
    let s1 = String::from("hello");
    let (s1, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() 返回字符串长度
    (s, length)
}
