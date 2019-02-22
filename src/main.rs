#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate tera;
extern crate config;
extern crate serde_yaml;
extern crate clap;
extern crate toml;

use std::fs;
use std::sync::Arc;
use std::collections::HashMap;
use std::io::prelude::*;
use std::fs::metadata;
use config::Value;
use tera::{Tera, Context, GlobalFn};
use clap::{Arg, App, SubCommand};

mod renderer;
mod config_parser;
mod structure_builder;

use config_parser::config_database::ConfigDatabase;

mod template_builder;

use template_builder::tera_builder::build_tera;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct CommandArgs<'a> {
    config: &'a str,
    template: &'a str,
    output: &'a str,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceNameItem {
    pub default_name: String,
    pub hyphen_name: Option<String>,
    pub snake_name: Option<String>,
    pub camel_name: Option<String>
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceFieldItem {
    pub name_info: ResourceNameItem,
    pub type_info: HashMap<String, String>
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceItem {
    pub name: String,
    pub api_class_name: Option<String>,
    pub biz_class_name: Option<String>,
    pub core_class_name: Option<String>,
    pub dal_class_name: Option<String>,
    pub sql_table_name: Option<String>,
    pub fields: Vec<ResourceFieldItem>
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct BuildConfig {
    pub group: String,
    pub project: String,
    pub resources: Option<HashMap<String, ResourceItem>>,
    pub ext: Option<HashMap<String, String>>
}

fn gen_get_config_val_fn(config_database: Box<ConfigDatabase>, val_type: &str) -> GlobalFn {
    return Box::new(move |args: HashMap<String, tera::Value>| -> tera::Result<tera::Value> {
        match args.get("key") {
            Some(key) => match tera::from_value::<String>(key.clone()) {
                Ok(key) => Ok(tera::to_value(config_database.get_str(key.as_str())).unwrap()),
                Err(_) => Err("oops".into()),
            },
            None => Err("oops".into()),
        }
    });
}

//fn gen_get_config_map_val_fn(config_database: Box<ConfigDatabase>, val_type: &str) -> GlobalFn {
//    return Box::new(move |args: HashMap<String, tera::Value>| -> tera::Result<tera::Value> {
//        match args.get("key") {
//            Some(key) => match tera::from_value::<String>(key.clone()) {
//                Ok(key) => {
//                    let val_map:HashMap<String, config::Value> = config_database.config.get_table(key.as_str()).unwrap().into();
//                    return Ok(tera::to_value(val_map).unwrap());
//                },
//                Err(_) => Err("oops".into()),
//            },
//            None => Err("oops".into()),
//        }
//    });
//}

fn render_project_structure(command_args: &CommandArgs, build_context: &BuildConfig, structure_path: &str) {
    let build_config = config_parser::config_database::build_config_database(command_args.config);
    let mut build_fn_map = HashMap::new();
    build_fn_map.insert("get_str", gen_get_config_val_fn(build_config, "str"));
//    let build_config2 = config_parser::config_database::build_config_database(command_args.config);
//    build_fn_map.insert("get_map", gen_get_config_map_val_fn(build_config2, "str"));

    let structure_template_dir = format!("{}/structure", command_args.template);
//    dbg!(&structure_template_dir);
    let tera_instance = build_tera(structure_template_dir.as_str(), build_fn_map);
    let mut context = Context::new();
    context.insert("context", &build_context);
    renderer::render_template::render_templates(&tera_instance, &context, &build_context,structure_template_dir.as_str(), structure_path);
}

fn create_project_structure(command_args: &CommandArgs, structure_path: &str, project_template_path: &str) {
    let mut project_structure = config::Config::default();
    project_structure.merge(config::File::with_name(format!( "{}/structure.toml", structure_path).as_str()));
    let structure_map = project_structure.try_into::<HashMap<String, Vec<Vec<String>>>>().unwrap();
    let structure_item_arr = structure_map.get("project_structure").unwrap();
    for structure_item in structure_item_arr {
//        assert_eq!(structure_item.len()>3, true);
        let template_dir_dot_style = structure_item.get(0).unwrap();
        let template_dir = format!("{}/{}", project_template_path, template_dir_dot_style.replace(".", "/"));
        println!("path: {}", &template_dir);
        match metadata(&template_dir) {
            Ok(f)=> println!("template dir already exists"),
            Err(e) => {
                match fs::create_dir_all(&template_dir) {
                    Ok(t)=> println!("template dir {} created.", &template_dir),
                    Err(e)=> panic!(e)
                }
            }
        };

        let file_name = structure_item.get(1).unwrap();
        println!("template file name: {}", &file_name);
        let template_file_name = format!("{}/{}", template_dir, file_name);
        match metadata(&template_file_name) {
            Ok(f)=> println!("template file already exists"),
            Err(e) => {
                let template_name = format!("{}/template/{}", command_args.template, structure_item.get(2).unwrap());
                println!("copy {} to {}", &template_name, &template_file_name);
                let template_content = fs::read_to_string(&template_name).unwrap();
                let mut extra_content = "";
                if structure_item.len() > 3 {
                    extra_content = structure_item.get(3).unwrap();
                }
                let full_content = format!("{}{}", &extra_content, &template_content);
                fs::write(template_file_name, full_content);

//                match fs::copy(&template_name, &template_file_name) {
//                    Ok(t)=> println!("template file {} created.", &template_file_name),
//                    Err(e)=> panic!(e)
//                }
            }
        };


        println!("template: {}", structure_item.get(2).unwrap());
    }
}

fn render_project(command_args: &CommandArgs, build_context: &BuildConfig, project_template_path: &str) {
    let config_database = config_parser::config_database::build_config_database(command_args.config);
    let mut fn_map = HashMap::new();
    fn_map.insert("get_str", gen_get_config_val_fn(config_database, "str"));

    let tera_instance = build_tera(project_template_path, fn_map);
    let mut context = Context::new();
    context.insert("context", &build_context);
    renderer::render_template::render_templates(&tera_instance, &context, &build_context, project_template_path, command_args.output)
}

fn main() {
    let matches = App::new("bullet")
        .version("0.1.3")
        .author("Wang Wei. <soulww@163.com>")
        .about("This is a generator for java server application write in rust.")
        .arg(Arg::with_name("dummy")
            .hidden(true)
            .possible_value("bullet"))
        .arg(Arg::with_name("config")
            .short("c")
            .long("config")
            .value_name("CONFIG")
            .help("Sets the config_database file")
            .required(true)
            .takes_value(true))
//        .arg(Arg::with_name("build")
//            .short("b")
//            .long("build")
//            .value_name("BUILD")
//            .help("Set the build file")
//            .required(true)
//            .takes_value(true))

//        .arg(Arg::with_name("structure")
//                 .short("s")
//                 .long("structure")
//                 .value_name("STRUCTURE")
//                 .help("Set the template structure")
//                 .required(true)
//                 .takes_value(true))

        .arg(Arg::with_name("template")
            .short("t")
            .long("template")
            .value_name("TEMPLATE")
            .help("Sets the template dir")
            .required(true)
            .takes_value(true))

        .arg(Arg::with_name("output")
            .short("o")
            .long("output")
            .value_name("OUTPUT")
            .help("Sets the output dir")
            .required(true)
            .takes_value(true))
        .get_matches();

    let command_args = CommandArgs {
//        build: matches.value_of("build").unwrap(),
//        structure: matches.value_of("structure").unwrap(),
        config: matches.value_of("config").unwrap(),
        template: matches.value_of("template").unwrap(),
        output: matches.value_of("output").unwrap(),
    };
//    dbg!(&command_args);

    let build_config_str = fs::read_to_string(command_args.config).unwrap();
    let build_config: BuildConfig = toml::from_str(build_config_str.as_str()).unwrap();
//    dbg!(&build_config);

    fs::remove_dir_all(".bullet_work_dir");
    let structure_path = ".bullet_work_dir/structure";
    let project_template_path = ".bullet_work_dir/project";
    render_project_structure(&command_args, &build_config, structure_path);
    create_project_structure(&command_args, structure_path, project_template_path);
    render_project(&command_args, &build_config, project_template_path);
//    fs::remove_dir_all(".bullet_work_dir");
}
