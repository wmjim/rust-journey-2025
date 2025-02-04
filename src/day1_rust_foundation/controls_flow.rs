use super::functions;

#[allow(dead_code)]
fn check_number(num: i32) {
    if num < 0 {
        println!("Negative");
    } else if num == 0 {
        println!("Zero");
    } else {
        println!("Positive");
    }
}

#[allow(dead_code)]
fn sum() -> u32 {
    let mut sum = 0;
    for i in 1..=100 {
        sum += i;
    }
    sum
}

#[allow(dead_code)]
fn while_fibonacci(n: i32) {
    while n > 0 {
        println!("{} ", functions::fibonacci(n));
    }
    println!("");
}