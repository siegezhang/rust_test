#[cfg(test)]
mod test6 {

    #[test]
    fn test_vector() {
        //let mut v=Vec::new();
        let mut v = vec![1, 2, 3, 5];
        v.push(3);
        let third = &v[2];
        print!("third number is {}\n", third);
        match v.get(2) {
            Some(third) => print!("third number is {}\n",third),
            None=>print!("there is no elements\n")
        }
    }
}
