#[cfg(test)]
mod test1 {
    use chrono::prelude::*;
    use mysql::prelude::*;
    use mysql::*; // 用来处理日期
    #[test]
    fn mysql_conn() {
        // println!("Hello, world!");
        let url = "mysql://root:123456@localhost:3306/rust_demo";
        let pool = Pool::new(url).unwrap(); // 获取连接池
        let mut conn = pool.get_conn().unwrap(); // 获取链接
        conn.query_iter("select * from tm_user")
            .unwrap()
            .for_each(|row| {
                let r: (
                    String,
                    NaiveDateTime,
                    NaiveDateTime,
                    String,
                    String,
                    String,
                    String,
                    String,
                    String,
                    i32,
                ) = from_row(row.unwrap());
                println!(
                    "{}, {:?},{:?},{}, {},{}, {},{},{:?},{}",
                    r.0, r.1, r.2, r.3, r.4, r.5, r.6, r.7, r.8, r.9
                );
            });
    }
}
