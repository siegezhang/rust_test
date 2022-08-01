#[cfg(test)]
mod test2 {
    use rbatis::{crud::CRUD, rbatis::Rbatis};

    #[tokio::test]
    async fn rbatis_conn() {
        let rb = Rbatis::new();
        ///连接数据库,自动判断驱动类型"mysql://*","postgres://*","sqlite://*","mssql://*"加载驱动   
        rb.link("mysql://root:123456@localhost:3306/rust_demo")
            .await
            .unwrap();
        //自定义连接池参数。(可选)
        // use crate::core::db::DBPoolOptions;
        // let mut opt =DBPoolOptions::new();
        // opt.max_connections=100;
        // rb.link_opt("mysql://root:123456@localhost:3306/test",&opt).await.unwrap();

        //启用日志输出，你也可以使用其他日志框架，这个不限定的
        //启用日志输出 comment fast_log::init_log("requests.log", 1000, log::Level::Info, None, true);
    }
}
