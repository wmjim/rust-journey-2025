pub fn print_variables() {
    // 定义一个不可变变量
    let x = 5;
    // 定义一个可变变量
    let mut y = 10;
    println!("x: {}, y: {}", x, y);
    y = 20;
    println!("updated y: {}", y);
}