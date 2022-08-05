#[cfg(test)]
macro_rules! create_impl {

  ($struct_name:ident, $({ $($function:tt)* }),*) => {
      struct $struct_name {
      }

      impl $struct_name {
          $($($function)*)*
      }
  };
}

#[test]
fn test4() {
    create_impl!(StructName, { fn foo(&self) -> u32 { return 432 } }, { fn bar(&self) -> u32 { return 765 } });
    let s=StructName{};
    s.foo();
    s.bar();
}