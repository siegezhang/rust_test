fn greet(name: &str) {
    println!("Hello, {}!", name);
}

#[test]
fn test8() {
    let name = "Pascal";
    greet(name);
}