mod test1;
mod test2;
mod test3;
mod test4;
mod test6;
mod test5;
mod test7;
mod test8;
mod test9;

#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

fn main() {
    let x: u32 = 12;
    if x > 2 && x < 20 { println!("12") }
    accept_both("test1");
    let a = String::from("John Doe");
    println!("{:?}", Person { name: a, age: 30 });
}


fn accept_both<S: Into<String>>(s: S) {
    let s = s.into();
    println!("test:{:}", s)
}


