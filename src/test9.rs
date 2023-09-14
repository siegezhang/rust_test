#[test]
fn test9() {
    let arr_of_arrs = [
        [1, 2, 3],
        [4, 5, 6],
        [7, 8, 9],
    ];
    let one_to_nine = arr_of_arrs.iter().flatten().collect::<Vec<_>>();
    println!("{:?}", one_to_nine);
}