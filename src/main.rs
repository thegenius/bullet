#[macro_use]
extern crate tera;

#[macro_use]
extern crate serde_derive;

extern crate clap;
extern crate config;
extern crate dirs;
extern crate git2;
extern crate serde_yaml;
extern crate toml;

mod build_config_parser;
mod command_args_parser;
mod structure_builder;
mod template_installer;
mod template_renderer;
mod tera_builder;

use command_args_parser::{BuildArg, InstallArg};

fn main() {
    let command_args: (Option<InstallArg>, Option<BuildArg>) =
        command_args_parser::parse_command_line_args();
    match command_args.0 {
        None => (),
        Some(install_arg) => {
            template_installer::install_template_from_git(install_arg.name, install_arg.url);
        }
    };

    match command_args.1 {
        None => (),
        Some(build_arg) => {
            template_renderer::render(build_arg);
        }
    }
}
