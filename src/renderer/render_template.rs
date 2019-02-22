use std::fs;
use std::fs::File;
use std::fs::DirEntry;
use std::path::Path;
use std::error::Error;
use std::path::PathBuf;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::BufRead;
use walkdir::WalkDir;
use tera::{Context, Tera};
use super::super::BuildConfig;

fn gen_target_path(entry: &walkdir::DirEntry, old_prefix: &str, new_prefix: &str) -> PathBuf {
    let strip_path = entry.path().strip_prefix(old_prefix).unwrap();
    return Path::new(new_prefix).join(strip_path);
}

pub fn walk_templates(template_path: &str) {
    for entry in WalkDir::new(template_path) {
        let entry = entry.unwrap();
        if entry.path().is_dir() {
            println!("dir {} already exists", entry.path().display());
        } else {
            println!("{}, {}", entry.file_type().is_dir(), entry.path().display());
        }
    }
}

fn read_template_resource_name(file_path: &Path) -> Option<String> {
    if file_path.is_file() {
        let f = File::open(file_path).unwrap();
        let file = BufReader::new(&f);
        for (num, line) in file.lines().enumerate() {
            let line_str = line.unwrap();
            if num == 0 {
                let mut iter = line_str.split_whitespace();
                if iter.next() == Some("{#") {
                    if iter.next() == Some("resource") {
                        if iter.next() == Some("=") {
                            return Some(iter.next().unwrap().to_string());
                        }
                    }
                }
            }
        }
    }
    return None;
}

pub fn render_templates(tera_instance: &Tera, context: &Context, build_context: &BuildConfig, template_path: &str, target_path: &str) {
    for entry in WalkDir::new(template_path) {
        let entry = entry.unwrap();
        let target_path = gen_target_path(&entry, template_path, target_path);
        if entry.path().is_dir() {
            if !target_path.exists() {
                match fs::create_dir_all(&target_path) {
                    Err(why) => panic!("{:?}", why),
                    Ok(()) => println!("create dir {} success!", target_path.display())
                }
            }
        } else {
            let resource_name: Option<String> = read_template_resource_name(entry.path());
//            dbg!(&resource_name);
            let striped_path = entry.path().strip_prefix(template_path).unwrap();
            let template_name = striped_path.to_str().unwrap().replace("\\", "/");
//            dbg!(&entry);
//            dbg!(&template_name);
            let mut render_context = context.clone();
            match resource_name {
                Some(name) => {
                    match &build_context.resources {
                        Some(map) => render_context.insert("resource", map.get(name.as_str()).unwrap()),
                        None => println!("no resources properties")
                    }
//                        let resources_map = build_context.resources.unwrap();
                }
                None => println!("no resource name found")
            }

            let content = tera_instance.render(template_name.as_str(), &render_context).unwrap();
            match fs::write(&target_path, content) {
                Err(why) => panic!("{:?}", why),
                Ok(()) => println!("render {} success!", target_path.display())
            }
        }
    }
}