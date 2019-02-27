use std::collections::HashMap;
use std::fs;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceNameItem {
    pub default_name: String,
    pub snake_name: Option<String>,
    pub hyphen_name: Option<String>,
    pub upper_camel_name: Option<String>,
    pub lower_camel_name: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceFieldItem {
    pub name_info: ResourceNameItem,
    pub type_info: HashMap<String, String>,
    pub ext: Option<HashMap<String, String>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceItem {
    pub name_info: ResourceNameItem,
    pub type_info: HashMap<String, String>,
    pub ext: Option<HashMap<String, String>>,
    pub fields: Vec<ResourceFieldItem>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct BuildConfig {
    pub group: String,
    pub project: String,
    pub ext: Option<HashMap<String, String>>,
    pub resources: Option<HashMap<String, ResourceItem>>,
}

pub fn parse_build_config_file(config_file_path: &String) -> BuildConfig {
    let build_config_str = match fs::read_to_string(config_file_path) {
        Err(why) => panic!("could not read file: {}, {}", config_file_path, why),
        Ok(config_str) => config_str,
    };
    return match toml::from_str(build_config_str.as_str()) {
        Err(why) => panic!("could not parse config file: {}, {}", config_file_path, why),
        Ok(build_config) => build_config,
    };
}
