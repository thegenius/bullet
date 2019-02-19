use std::fs;
use std::fs::File;
use std::fs::DirEntry;
use std::path::Path;
use std::error::Error;
use std::path::PathBuf;
use std::io::prelude::*;
use walkdir::WalkDir;
use tera::{Context, Tera};

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

pub fn render_templates(tera_instance: &Tera, context: &Context, template_path: &str, target_path: &str) {
    for entry in WalkDir::new(template_path) {
        let entry = entry.unwrap();
        let target_path = gen_target_path(&entry, template_path, target_path);
        if entry.path().is_dir() {
            if !target_path.exists() {
                match fs::create_dir(&target_path) {
                    Err(why) => panic!("{:?}", why),
                    Ok(())=> println!("create dir {} success!", target_path.display())
                }
            }
        } else {
            let striped_path = entry.path().strip_prefix(template_path).unwrap();
            let template_name = striped_path.to_str().unwrap().replace("\\", "/");
            dbg!(&entry);
            dbg!(&template_name);
            let content = tera_instance.render(template_name.as_str(), context).unwrap();
            match fs::write(&target_path, content) {
                Err(why) => panic!("{:?}", why),
                Ok(())=> println!("render {} success!", target_path.display())
            }
        }
    }
}