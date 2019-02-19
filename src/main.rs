#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate tera;
extern crate config;
extern crate serde_yaml;
extern crate clap;

use std::sync::Arc;
use std::collections::HashMap;
use std::io::prelude::*;
use config::Value;
use tera::{Tera, Context, GlobalFn};
use clap::{Arg, App, SubCommand};

mod renderer;
mod config_parser;

use config_parser::config_database::ConfigDatabase;

mod template_builder;

use template_builder::tera_builder::build_tera;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct CommandArgs<'a> {
    config: &'a str,
    template: &'a str,
    output: &'a str,
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

fn main() {
    let matches = App::new("bullet")
        .version("1.0")
        .author("Wang Wei. <soulww@163.com>")
        .about("This is a generator for java server application write in rust.")
        .arg(Arg::with_name("config")
            .short("c")
            .long("config")
            .value_name("CONFIG")
            .help("Sets the config_database file")
            .required(true)
            .takes_value(true))

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
        config: matches.value_of("config").unwrap(),
        template: matches.value_of("template").unwrap(),
        output: matches.value_of("output").unwrap(),
    };
    dbg!(&command_args);

    let config_database = config_parser::config_database::build_config_database(command_args.config);
    let mut fn_map = HashMap::new();
    fn_map.insert("get_str", gen_get_config_val_fn(config_database, "str"));

    let tera_instance = build_tera(command_args.template, fn_map);
    let mut context = Context::new();
    renderer::render_template::render_templates(&tera_instance, &context, command_args.template, command_args.output)
}
