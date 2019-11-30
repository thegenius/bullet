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
mod photo;

use command_args_parser::{BuildArg, CreateArg, InstallArg, FastCreateArg};

fn main() {
    let command_args: (Option<InstallArg>, Option<BuildArg>, Option<CreateArg>, Option<FastCreateArg>) =
        command_args_parser::parse_command_line_args();
    match command_args.0 {
        None => (),
        Some(install_arg) => {
            template_installer::install_template_from_git(
                install_arg.name,
                install_arg.url,
                install_arg.force,
            );
        }
    };

    match command_args.1 {
        None => (),
        Some(build_arg) => {
            template_renderer::render(build_arg);
        }
    }

    match command_args.2 {
        None => (),
        Some(create_arg) => {
            template_installer::create_build_config_from_installed(create_arg.name);
        }
    }

    match command_args.3 {
        None => (),
        Some(fast_create_arg) => {
            match template_installer::fetch_template_path(&fast_create_arg.name) {
                None => panic!("could not find the template of {}", &fast_create_arg.name),
                Some(template_path) => {
                    template_renderer::render_by_template(template_path.as_str(), &fast_create_arg.build_config, ".");
                }
            }
        }
    }
}
