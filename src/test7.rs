use proc_macro_define_crate::mytest_proc_macro;

#[mytest_proc_macro(blog(::ideawand::com))]
fn pro_foo(a:i32){
    println!("hello, blog.ideawand.com, hello, 极客幼稚园");
}



#[test]
fn test7() {
    //过程宏的处理
}