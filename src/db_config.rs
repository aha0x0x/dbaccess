use url::Url;

#[derive(Debug)]
pub struct DbConfig {
    pub url: Url,
    pub username: String, 
    pub password: String
}

impl DbConfig {
    pub fn new(url: String, user: String, password: String) -> DbConfig {
        let parsed = Url::parse(&url).unwrap();
        DbConfig { url: parsed, username: user, password: password }
    }
}