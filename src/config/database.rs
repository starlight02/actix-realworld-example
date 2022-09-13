use rbatis::Rbatis;
use rbdc_pg::driver::PgDriver;
use crate::config::CONFIG;

pub fn init_pool() -> Rbatis {
    #[cfg(debug_assertions)]
    let db_url = {
        println!("rbatis pool init ({})...", CONFIG.DB_URL);
        CONFIG.DB_URL.as_str()
    };

    #[cfg(not(debug_assertions))]
    let db_url = {
        env!("ACTIX_DB_URL", "Environment variable \"ACTIX_DB_URL\" not found!")
    };

    let rbatis = Rbatis::new();
    rbatis
        .init(PgDriver {}, db_url)
        .expect("rbatis pool init fail!");

    rbatis
}
