macro_rules! mytest6 {
    ($s:tt) => {{
        $s();
         println!("{:?}",stringify!($s));
    }};
}



#[test]
fn test5() {
    mytest6!({pub fn foo1() { println!("hello1111111113") }});
}