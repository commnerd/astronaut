struct Config {
    
}

impl Config {
    pub fn new() {
        Config{}
    }

    pub fn get(&self, key: &'static str) -> Any {
        
    }

    pub fn set(self, key: &'static str, val: Any) {

    }

    pub fn populate(self, section: &'static str, config: Config) -> Config {
        config
    }
}