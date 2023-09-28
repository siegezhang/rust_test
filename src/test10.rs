fn curry_add(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x + y
}

#[test]
fn test9() {
    let add_nine = curry_add(9);
    let result = add_nine(1);
    println!("Result: {}", result);
}