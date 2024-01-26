fn main() {
    {
        // s 在这里无效
        let s = "hello"; // 从这里开始，s 是有效的
                         // 使用 s
                         // ...
    } // 此作用域已结束，s 不再有效

    // let mut s = String::from("hello");
    // s.push_str(", world!"); // push_str() 在字符串后追加字面值
    // println!("{}", s); // 打印 hello, world!

    {
        let s = String::from("hello"); // 从此处起，s 是有效的
                                       // 使用 s
                                       // ...
    } // 此作用域已经结束，s 不再有效
}
