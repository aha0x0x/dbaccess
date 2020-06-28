
mod app_config;
mod db_config;

fn main() {
    let aaa = app_config::AppConfig::new();
    let db = aaa.db_config();
    print!("{:?}", db);
}
