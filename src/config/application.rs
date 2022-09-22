#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ApplicationConfig {
    // actix 绑定的 host
    pub BIND_HOST: String,
    // actix 绑定的端口
    pub PORT: u32,
    // 数据库 host
    pub DB_URL: String,
    // 签名 token 的 secret
    pub TOKEN_SECRET: String,
    // 签名 token 的 Issuer
    pub TOKEN_ISSUER: String,
}

///默认配置
impl Default for ApplicationConfig {
    fn default() -> Self {
        let yml_data = include_str!("../../application.yaml");
        //读取配置
        let result: ApplicationConfig = serde_yaml::from_str(yml_data).expect("load config file fail");
        // if result.debug {
        //     println!("[abs_admin] load config:{:?}", result);
        //     println!("[abs_admin] ///////////////////// Start On Debug Mode ////////////////////////////");
        // } else {
        //     println!("[abs_admin] ///////////////////// Start On Release Mode ////////////////////////////");
        // }
        result
    }
}
