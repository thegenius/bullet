use tera::Context;
use config::Value;
use config::Source;
use config::ConfigError;
use walkdir::WalkDir;
use std::fs::metadata;
use std::collections::hash_map::HashMap;

#[derive(Debug)]
pub struct ConfigDatabase {
    config: config::Config
}

impl ConfigDatabase {
    pub fn get_int(&self, key: &str) -> i64 {
        self.config.get_int(key).unwrap()
    }
    pub fn get_float(&self, key: &str) -> f64 {
        self.config.get_float(key).unwrap()
    }
    pub fn get_bool(&self, key: &str) -> bool {
        self.config.get_bool(key).unwrap()
    }
    pub fn get_str(&self, key: &str) -> String {
        self.config.get_str(key).unwrap()
    }
//    pub fn get(&self, key: &str) -> Result<T, ConfigError> {
//        self.config.get(key)
//    }
    pub fn convert_into_map(&self) -> HashMap<String, Value> {
       return self.config.collect().unwrap();
    }
    pub fn convert_into_tera_context(&self) -> Context {
        let context: Context = Context::new();
//        let content_map = self.convert_into_map();
//        dbg!(&content_map);
//        for (key,val) in content_map {
//            match val {
//                <config::Value>::ValueKind::Nil => println!("Nil"),
//                _ => println!("default")
//            }
//        }
        return context;
    }
}

pub fn build_config_database(config_dir: &str) -> Box<ConfigDatabase> {
    let mut database = Box::new(ConfigDatabase {
        config: config::Config::default()
    });

    let md = metadata(config_dir).unwrap();
    if md.is_file() {
        database.config.merge(config::File::with_name(config_dir));
        return database;
    }

    for entry in WalkDir::new(config_dir) {
        let entry = entry.unwrap();
        if !entry.path().is_dir() {
            database.config.merge(config::File::with_name(entry.path().to_str().unwrap()));
        }
    }
    return database;
}