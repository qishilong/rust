fn main() {
    // let number = 7;
    // if number < 5 {
    //     println!("true");
    // } else {
    //     println!("false");
    // }

    // let number = 5;
    // // 报错 必须要 bool 类型
    // if number {
    //     println!("number")
    // }

    // let number = 6;
    // if number % 4 == 0 {
    //     println!("number is divisible by 4");
    // } else if number % 3 == 0 {
    //     println!("number is divisible by 3");
    // } else if number % 2 == 0 {
    //     println!("number is divisible by 2");
    // } else {
    //     println!("number is not divisible by 4, 3, or 2");
    // }

    // let 语句中使用 if
    // let condition = true;
    // let number = if condition { 5 } else { 6 };
    // println!("the value of number is: {number}");

    // 下面代码会报错
    /*let condition = true;
    let number = if condition{5} else {'six'};
    println!("The value of number is: {number}");*/

    // loop {
    //     println!("again!")
    // }

    // break
    // let mut counter = 0;
    // let result = loop {
    //     counter += 1;
    //     if counter == 10 {
    //         break counter * 2;
    //     }
    // };
    // println!("The result is {result}");

    // 循环标签
    // let mut count = 0;
    // 'counting_up: loop {
    //     println!("count = {count}");
    //     let mut remaining = 10;
    //     loop {
    //         println!("remaining = {remaining}");
    //         if remaining == 9 {
    //             break;
    //         }
    //         if count == 2 {
    //             break 'counting_up;
    //         }
    //         remaining -= 1;
    //     }
    //     count += 1;
    // }
    // println!("End count = {count}");

    // while
    // let mut number = 3;
    // while number != 0 {
    //     println!("{number}");
    //     number -= 1;
    // }
    // println!("OFF");

    // while 遍历
    /*let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }*/

    // for 遍历
    /*let number = [10, 20, 30, 40, 50];
    for element in number {
        println!("the value is : {element}");
    }*/

    // rev 反转 range
    /*for number in (1..4).rev() {
        println!("{number}");
    }
    println!("OFF");*/

    for i in (0..4).rev() {
        println!("{i}");
    }
    println!("OFF");
}
