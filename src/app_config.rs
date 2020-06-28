
use config;
use crate::db_config::DbConfig;

#[derive(Debug)]
pub struct AppConfig {
    pub config: config::Config
}

impl AppConfig {
    pub fn new() -> AppConfig {
        let mut config = config::Config::default();
        config
        .merge(config::File::with_name("src/resources/config.json")).unwrap();
        AppConfig { config: config }
    }

    pub fn db_config( &self ) -> DbConfig {
        let url = self.config.get_str("datasource.url").unwrap();
        let username = self.config.get_str("datasource.username").unwrap();
        let password = self.config.get_str("datasource.password").unwrap();
        DbConfig::new( url, username, password )
    }
}