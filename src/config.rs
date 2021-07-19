use std::sync::Mutex;

lazy_static! {
    static ref CONFIG: Mutex<Config> = Mutex::new(Default::default());
}

pub fn get_config() -> Config {
    CONFIG.lock().unwrap().clone()
}

pub fn set_config(config: Config) {
    let mut global_conf = CONFIG.lock().unwrap();
    let _ = std::mem::replace(&mut *global_conf, config);
}

#[derive(Clone, Debug)]
pub struct Config {
    pub itemdb_url: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            itemdb_url: "https://raw.githubusercontent.com/broderickhyman/ao-bin-dumps/master/formatted/items.txt".to_owned()
        }
    }
}
