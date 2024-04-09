pub struct Config {
    pub addr: String,
    pub port: u16,
    pub log_level: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            addr: "0.0.0.0".to_string(),
            port: 8000,
            log_level: "info".to_string(),
        }
    }
}
