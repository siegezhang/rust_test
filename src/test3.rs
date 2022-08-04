#[cfg(test)]
macro_rules! map {
    ($( $key:expr=>$value:expr),*) => {{
        let mut hm=std::collections::HashMap::new();
        $(  hm.insert($key, $value);)*
        hm
    }};
}

#[test]
fn test3() {
    let user = map!(
        "name"=>"Finn",
        "gender"=>"Boy"
    );
    println!("user {:?}", user);
}