#[cfg(test)]
macro_rules! mytest5_1 {
    ($s:ident) => {{
        $s();
        // $s();
        // println!("mytest1");
        // println!("{:?}",stringify!($s));
    }};
}

macro_rules! mytest5_2 {
    ($s:expr) => {
        println!($s);
    };
}

#[test]
fn test5() {
    pub fn foo1() { println!("hello1111111111") }
    mytest5_1!( foo1);
    mytest5_2!("adadasd");
}