use super::build_config_parser;
use super::build_config_parser::BuildConfig;
use super::command_args_parser::BuildArg;
use super::structure_builder;
use super::structure_builder::ProjectStructureItem;
use super::template_installer;
use super::tera_builder;
use std::fs;
use std::path::Path;
use tera::{Context, Tera};

// fn gen_target_path(entry: &walkdir::DirEntry, old_prefix: &str, new_prefix: &str) -> PathBuf {
//     let strip_path = entry.path().strip_prefix(old_prefix).unwrap();
//     return Path::new(new_prefix).join(strip_path);
// }

// pub fn walk_templates(template_path: &str) {
//     for entry in WalkDir::new(template_path) {
//         let entry = entry.unwrap();
//         if entry.path().is_dir() {
//             println!("dir {} already exists", entry.path().display());
//         } else {
//             println!("{}, {}", entry.file_type().is_dir(), entry.path().display());
//         }
//     }
// }

// fn read_template_resource_name(file_path: &Path) -> Option<String> {
//     if file_path.is_file() {
//         let f = File::open(file_path).unwrap();
//         let file = BufReader::new(&f);
//         for (num, line) in file.lines().enumerate() {
//             let line_str = line.unwrap();
//             if num == 0 {
//                 let mut iter = line_str.split_whitespace();
//                 if iter.next() == Some("{#") {
//                     if iter.next() == Some("resource") {
//                         if iter.next() == Some("=") {
//                             return Some(iter.next().unwrap().to_string());
//                         }
//                     }
//                 }
//             }
//         }
//     }
//     return None;
// }

// pub fn render_templates(
//     tera_instance: &Tera,
//     context: &Context,
//     build_context: &BuildConfig,
//     template_path: &str,
//     target_path: &str,
// ) {
//     for entry in WalkDir::new(template_path) {
//         let entry = entry.unwrap();
//         let target_path = gen_target_path(&entry, template_path, target_path);
//         if entry.path().is_dir() {
//             if !target_path.exists() {
//                 match fs::create_dir_all(&target_path) {
//                     Err(why) => panic!("{:?}", why),
//                     Ok(()) => println!("create dir {} success!", target_path.display()),
//                 }
//             }
//         } else {
//             let resource_name: Option<String> = read_template_resource_name(entry.path());
//             //            dbg!(&resource_name);
//             let striped_path = entry.path().strip_prefix(template_path).unwrap();
//             let template_name = striped_path.to_str().unwrap().replace("\\", "/");
//             //            dbg!(&entry);
//             //            dbg!(&template_name);
//             let mut render_context = context.clone();
//             match resource_name {
//                 Some(name) => {
//                     match &build_context.resources {
//                         Some(map) => {
//                             render_context.insert("resource", map.get(name.as_str()).unwrap())
//                         }
//                         None => println!("no resources properties"),
//                     }
//                     //                        let resources_map = build_context.resources.unwrap();
//                 }
//                 None => println!("no resource name found"),
//             }

//             let content = tera_instance
//                 .render(template_name.as_str(), &render_context)
//                 .unwrap();
//             match fs::write(&target_path, content) {
//                 Err(why) => panic!("{:?}", why),
//                 Ok(()) => println!("render {} success!", target_path.display()),
//             }
//         }
//     }
// }

// fn gen_get_config_val_fn(config_database: Box<ConfigDatabase>, val_type: &str) -> GlobalFn {
//     return Box::new(move |args: HashMap<String, tera::Value>| -> tera::Result<tera::Value> {
//         match args.get("key") {
//             Some(key) => match tera::from_value::<String>(key.clone()) {
//                 Ok(key) => Ok(tera::to_value(config_database.get_str(key.as_str())).unwrap()),
//                 Err(_) => Err("oops".into()),
//             },
//             None => Err("oops".into()),
//         }
//     });
// }

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

// pub fn process_structure(structure_file: &str) {
//     let mut config = config::Config::default();
//     config.merge(config::File::with_name(structure_file));
//     let config_map = config
//         .try_into::<HashMap<String, Vec<Vec<String>>>>()
//         .unwrap();
//     let project_structure = config_map.get("project_structure").unwrap();
//     for structure_item in project_structure {
//         assert_eq!(structure_item.len(), 3);
//         let template_dir = structure_item.get(0).unwrap();
//         println!("path: {}", template_dir);
//         let dir_md = fs::metadata(template_dir).unwrap();
//         println!("{}", dir_md.is_file());
//         println!("name: {}", structure_item.get(1).unwrap());
//         println!("template: {}", structure_item.get(2).unwrap());
//     }
// }

// fn render_project_structure(template_path: &str, build_config: &BuildConfig, structure_path: &str) {
//     let structure_template_dir = format!("{}/structure", template_path);
//     let tera = tera_builder::build_tera(structure_template_dir.as_str());
//     let mut context = Context::new();
//     context.insert("context", &build_config);
//     render_templates(
//         &tera,
//         &context,
//         &build_config,
//         structure_template_dir.as_str(),
//         structure_path,
//     );
// }

// fn create_project_structure(
//     template_path: &str,
//     structure_path: &str,
//     project_template_path: &str,
// ) {
//     let mut project_structure = config::Config::default();
//     project_structure.merge(config::File::with_name(
//         format!("{}/structure.toml", structure_path).as_str(),
//     ));
//     let structure_map = project_structure
//         .try_into::<HashMap<String, Vec<Vec<String>>>>()
//         .unwrap();
//     let structure_item_arr = structure_map.get("project_structure").unwrap();
//     for structure_item in structure_item_arr {
//         //        assert_eq!(structure_item.len()>3, true);
//         let template_dir_dot_style = structure_item.get(0).unwrap();
//         let template_dir = format!(
//             "{}/{}",
//             project_template_path,
//             template_dir_dot_style.replace(".", "/")
//         );
//         println!("path: {}", &template_dir);
//         match fs::metadata(&template_dir) {
//             Ok(f) => println!("template dir already exists"),
//             Err(e) => match fs::create_dir_all(&template_dir) {
//                 Ok(t) => println!("template dir {} created.", &template_dir),
//                 Err(e) => panic!(e),
//             },
//         };

//         let file_name = structure_item.get(1).unwrap();
//         println!("template file name: {}", &file_name);
//         let template_file_name = format!("{}/{}", template_dir, file_name);
//         match fs::metadata(&template_file_name) {
//             Ok(f) => println!("template file already exists"),
//             Err(e) => {
//                 let template_name = format!(
//                     "{}/template/{}",
//                     template_path,
//                     structure_item.get(2).unwrap()
//                 );
//                 println!("copy {} to {}", &template_name, &template_file_name);
//                 let template_content = fs::read_to_string(&template_name).unwrap();
//                 let mut extra_content = "";
//                 if structure_item.len() > 3 {
//                     extra_content = structure_item.get(3).unwrap();
//                 }
//                 let full_content = format!("{}{}", &extra_content, &template_content);
//                 fs::write(template_file_name, full_content);

//                 //                match fs::copy(&template_name, &template_file_name) {
//                 //                    Ok(t)=> println!("template file {} created.", &template_file_name),
//                 //                    Err(e)=> panic!(e)
//                 //                }
//             }
//         };

//         println!("template: {}", structure_item.get(2).unwrap());
//     }
// }

// fn render_project(build_context: &BuildConfig, project_template_path: &str, output: &str) {
//     // let config_database = config_parser::config_database::build_config_database(command_args.config);
//     // let mut fn_map = HashMap::new();
//     // fn_map.insert("get_str", gen_get_config_val_fn(config_database, "str"));

//     let tera_instance = tera_builder::build_tera(project_template_path);
//     let mut context = Context::new();
//     context.insert("context", &build_context);
//     render_templates(
//         &tera_instance,
//         &context,
//         &build_context,
//         project_template_path,
//         output,
//     )
// }

fn render_structure_item(tera: &Tera, context: &Context, out: &str, item: &ProjectStructureItem) {
    let content = match tera.render(&item.item_tmpl, context) {
        Err(why) => panic!("failed to render item: {}", why),
        Ok(content) => content,
    };
    let file_path = item.item_path.replace(".", "/");
    let file_dir = format!("{}/{}", out, &file_path);
    template_installer::prepare_dir(Path::new(&file_dir));
    let file_name = format!("{}/{}", file_dir, &item.item_file);
    match fs::write(file_name, &content) {
        Err(why) => panic!("failed to write item: {}", why),
        Ok(_) => (),
    }
}

fn render_static_structure(
    tera: &Tera,
    build_config: &BuildConfig,
    out: &str,
    static_item: &ProjectStructureItem,
) {
    let mut context = Context::new();
    context.insert("context", &build_config);
    let item = structure_builder::parse_structure_item(static_item, &context);
    dbg!(&item);
    render_structure_item(tera, &context, out, &item);
}

fn render_dynamic_structure(
    tera: &Tera,
    build_config: &BuildConfig,
    out: &str,
    dynamic_item: &ProjectStructureItem,
) {
    println!("render dynamic");
    let resources = &build_config.resources;
    match resources {
        None => (),
        Some(resources) => {
            for (_, val) in resources {
                let mut context = Context::new();
                context.insert("context", &build_config);
                context.insert("resource", &val);
                let item = structure_builder::parse_structure_item(dynamic_item, &context);
                dbg!(&item);
                render_structure_item(tera, &context, out, &item);
            }
        }
    }
}

fn render_by_template(template_path: &str, build_config: &BuildConfig, output: &str) {
    let project_structure_items = structure_builder::fetch_project_structure_items(template_path);
    let template_dir = format!("{}/template", template_path);
    let tera = tera_builder::build_tera(&template_dir);
    for item in project_structure_items {
        if item.is_dynamic() {
            render_dynamic_structure(&tera, &build_config, output, &item);
        }
        if item.is_static() {
            render_static_structure(&tera, &build_config, output, &item);
        }
    }
}

pub fn render(build_arg: BuildArg) {
    match build_arg {
        BuildArg::NamedBuildArg { config, name, out } => {
            let build_config = build_config_parser::parse_build_config_file(&config);
            match template_installer::fetch_template_path(&name) {
                None => panic!("could not find the template of {}", &name),
                Some(template_path) => {
                    render_by_template(template_path.as_str(), &build_config, out.as_str());
                }
            }
        }
        BuildArg::TemplatedBuildArg {
            config,
            template,
            out,
        } => {
            let build_config = build_config_parser::parse_build_config_file(&config);
            render_by_template(template.as_str(), &build_config, out.as_str());
        }
    }
}
