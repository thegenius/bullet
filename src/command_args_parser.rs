use clap::{App, Arg, ArgGroup, ArgMatches, SubCommand};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum BuildArg {
    TemplatedBuildArg {
        config: String,
        template: String,
        out: String,
    },
    NamedBuildArg {
        config: String,
        name: String,
        out: String,
    },
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct InstallArg {
    pub name: String,
    pub url: String,
}

fn parse_build_args(build_args: &Option<&ArgMatches>) -> Option<BuildArg> {
    return match build_args {
        None => None,
        Some(build_command) => {
            let out_arg: String = build_command.value_of("output").unwrap().to_string();
            let build_config_arg: String = build_command.value_of("config").unwrap().to_string();
            let build_name_arg = build_command.value_of("name");
            if build_name_arg.is_some() {
                let named_build_arg: BuildArg = BuildArg::NamedBuildArg {
                    config: build_config_arg,
                    name: build_name_arg.unwrap().to_string(),
                    out: out_arg,
                };
                return Some(named_build_arg);
            } else {
                let build_template_arg = build_command.value_of("template").unwrap().to_string();
                return Some(BuildArg::TemplatedBuildArg {
                    config: build_config_arg,
                    template: build_template_arg,
                    out: out_arg,
                });
            }
        }
    };
}

fn parse_install_args(install_command: &Option<&ArgMatches>) -> Option<InstallArg> {
    return match install_command {
        None => None,
        Some(install_command) => {
            let name_arg: String = install_command.value_of("name").unwrap().to_string();
            let url_arg: String = install_command.value_of("url").unwrap().to_string();
            return Some(InstallArg {
                name: name_arg,
                url: url_arg,
            });
        }
    };
}

pub fn parse_command_line_args() -> (Option<InstallArg>, Option<BuildArg>) {
    let comand_line_matches = App::new("bullet")
        .version("0.1.3")
        .author("Wang Wei. <soulww@163.com>")
        .about("This is a generator for java server application write in rust.")
        .arg(
            Arg::with_name("dummy")
                .hidden(true)
                .possible_value("bullet"),
        )
        .subcommand(
            SubCommand::with_name("build")
                .about("build project from config and template")
                .group(
                    ArgGroup::with_name("build-arg")
                        .args(&["template", "name"])
                        .required(true),
                )
                .arg(
                    Arg::with_name("config")
                        .short("c")
                        .long("config")
                        .value_name("CONFIG")
                        .default_value("bullet.toml")
                        .help("Sets the config_database file")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("template")
                        .short("t")
                        .long("template")
                        .value_name("TEMPLATE")
                        .help("Sets the template dir")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("name")
                        .short("n")
                        .long("name")
                        .value_name("NAME")
                        .help("Sets the template name of installed")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("output")
                        .short("o")
                        .long("output")
                        .value_name("OUTPUT")
                        .help("Sets the output dir")
                        .required(true)
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("install")
                .about("install template from git")
                .arg(
                    Arg::with_name("name")
                        .long("name")
                        .short("n")
                        .help("set the template name")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("url")
                        .long("url")
                        .short("u")
                        .help("set the git url of the template")
                        .required(true)
                        .takes_value(true),
                ),
        )
        .get_matches();

    let build_arg: Option<BuildArg> =
        parse_build_args(&comand_line_matches.subcommand_matches("build"));
    let install_arg: Option<InstallArg> =
        parse_install_args(&comand_line_matches.subcommand_matches("install"));
    return (install_arg, build_arg);
}
