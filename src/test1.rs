#[cfg(test)]
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

#[test]
fn test1() {
    // Create functions named `foo` and `bar` with the above macro.
    create_function!(foo);
    create_function!(bar);
    foo();
    bar();
}