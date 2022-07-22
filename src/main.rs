macro_rules! create_function {
    // This macro takes an argument of designator `ident` and
    // creates a function named `$func_name`.
    // The `ident` designator is used for variable/function names.
    ($func_name:ident) => {
        fn $func_name() {
            // The `stringify!` macro converts an `ident` into a string.
            println!("You called {:?}()",
                     stringify!($func_name));
        }
    };
}

// Create functions named `foo` and `bar` with the above macro.
create_function!(foo);
create_function!(bar);

macro_rules! print_result {
    // This macro takes an expression of type `expr` and prints
    // it as a string along with its result.
    // The `expr` designator is used for expressions.
    ($expression:expr) => {
        // `stringify!` will convert the expression *as it is* into a string.
        println!("{:?} = {:?}",
                 stringify!($expression),
                 $expression);
    };
}
macro_rules! mytest {
    ($s:expr) => {
        println!($s);
    };
}
macro_rules! mytest1 {
    ($s:ident) => {{
        $s();
        // $s();
        // println!("mytest1");
        // println!("{:?}",stringify!($s));
    }};
}
macro_rules! mytest2 {
    ($s:tt) => {{
        $s();
         println!("{:?}",stringify!($s));
    }};
}
macro_rules! create_impl {

  ($struct_name:ident, $({ $($function:tt)* }),*) => {
      struct $struct_name {
      }

      impl $struct_name {
          $($($function)*)*
      }
  };
}
macro_rules! map {
    ($( $key:expr=>$value:expr),*) => {{
        let mut hm=std::collections::HashMap::new();
        $(  hm.insert($key, $value);)*
        hm
    }};
}



fn main() {
    foo();
    bar();
    pub fn foo1() { println!("hello1111111111") }
    mytest1!( foo1);
    mytest2!({pub fn foo1() { println!("hello1111111113") }});
    let mut map = std::collections::HashMap::new();
    map.insert("foo1", "122");
    let x: Vec<u32> = vec![1, 2, 3];
    //create_impl!(StructName, { fn foo() -> u32 { return 432 } }, { fn bar() -> u32 { return 765 } });
    // let user = map!(
    //     "name"=>"Finn",
    //     "gender"=>"Boy"
    // );
    // println!("user {:?}", user);
    // print_result!(1u32 + 1);
    // Recall that blocks are expressions too!
    print_result!({
        let x = 1u32;
        x * x + 2 * x - 1
    });
}
