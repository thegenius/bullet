use fs::File;
use git2::Repository;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct InstalledRecord {
    name: String,
    url: String,
    path: String,
}

pub fn prepare_dir(dir: &Path) {
    if dir.exists() {
        if dir.is_file() {
            panic!("{} already exists and is not a directory!", dir.display());
        }
    } else {
        match fs::create_dir_all(&dir) {
            Ok(_) => println!("directory: {} created!", &dir.display()),
            Err(why) => panic!("{}", why),
        }
    }
}

pub fn gen_template_path(name: &String) -> PathBuf {
    match dirs::home_dir() {
        Some(path) => {
            return Path::new(&path).join(".bullet_templates").join(name);
        }
        None => {
            panic!("could not find the home dir of the user, consider set the env variable HOME")
        }
    }
}

// pub fn prepare_templates_dir(dir: String) {
//     match dirs::home_dir() {
//         Some(path) => {
//             let template_dir = path.join(".bullet_templates").join(dir);
//             prepare_dir(&template_dir);
//         }
//         None => {
//             panic!("could not find the home dir of the user, consider set the env variable HOME")
//         }
//     }
// }

pub fn read_installed_records() -> HashMap<String, InstalledRecord> {
    let record_path = gen_template_path(&String::from("installed-record.toml"));
    let records = match fs::read_to_string(&record_path) {
        Err(_) => {
            // let record_path_string = record_path.to_str().unwrap().to_string();
            match File::create(&record_path) {
                Err(why) => panic!("couldn't open {}: {}", &record_path.display(), why),
                Ok(_) => {
                    println!("record file {} created!", &record_path.display());
                    String::from("")
                }
            }
        }
        Ok(content) => content,
    };

    let installed_record_map: HashMap<String, InstalledRecord> =
        toml::from_str(&records.as_str()).unwrap();
    return installed_record_map;
}

pub fn write_installed_records(records: HashMap<String, InstalledRecord>) {
    let record_path = gen_template_path(&String::from("installed-record.toml"));
    let content = toml::to_string(&records).unwrap();
    match fs::write(record_path, content) {
        Err(why) => panic!("failed to save install record: {}", why),
        Ok(_) => (),
    }
}

pub fn save_install_record(name: String, url: String, path: String) {
    let mut records = read_installed_records();
    let record_key = name.clone();
    let new_record = InstalledRecord {
        name: name,
        url: url,
        path: path,
    };
    records.insert(record_key, new_record);
    write_installed_records(records);
}

// pub fn remove_install_record(name: String) {
//     let mut records = read_installed_records();
//     records.remove(&name);
//     write_installed_records(records);
// }

pub fn fetch_template_path(name: &String) -> Option<String> {
    let records = read_installed_records();
    match records.get(name) {
        None => None,
        Some(record) => Some(String::from(record.path.as_str())),
    }
}

fn generate_repository_url(name: &String, url: &String) -> String {
    if name.starts_with("bullet-") {
        if !url.is_empty() {
            panic!("you can not set url for repository official, consider remove --url config");
        }
        if name.eq("bullet-spring-java") {
            return String::from("https://github.com/thegenius/bullet-spring-java.git");
        } else if name.eq("bullet-sql-accumulator") {
            return String::from("https://github.com/thegenius/bullet-sql-accumulator.git");
        } else if name.eq("bullet-kotlin-grpc") {
            return String::from("https://github.com/thegenius/bullet-kotlin-grpc.git");
        } else {
            panic!("sorry, you choose an official repository not supported for now!");
        }
    } else {
        if url.is_empty() {
            panic!(
                "please set the repository url that you want to install! consider use --url=<URL>"
            );
        }
        return url.clone();
    }
}

pub fn install_template_from_git(name: String, url: String, force: bool) {
    if name.is_empty() {
        panic!("install name is empty or url is empty!")
    }
    let template_path = gen_template_path(&name);
    let real_url = generate_repository_url(&name, &url);
    println!("template install path: {}", &template_path.display());
    if force {
        if template_path.exists() {
            match fs::remove_dir_all(&template_path) {
                Err(why) => panic!(
                    "failed to clear repository : {}, consider remove it manually",
                    why
                ),
                Ok(_) => (),
            }
        }
    }
    match Repository::clone(real_url.as_str(), &template_path) {
        Ok(_) => {
            println!(
                "install template {} to {} success!",
                &name.as_str(),
                &template_path.display()
            );
            let template_path_string: String = template_path.to_str().unwrap().to_string();
            save_install_record(name, url, template_path_string);
        }
        Err(e) => panic!("failed to clone git repository: {}", e),
    }
}

pub fn create_build_config_from_installed(name: String) {
    if name.is_empty() {
        panic!("create name is empty!")
    }
    let template_path = fetch_template_path(&name).unwrap();
    let example_file_path = format!("{}/bullet.toml", &template_path.as_str());
    match fs::copy(example_file_path, "bullet.toml") {
        Err(why) => panic!("failed to create bullet.toml from {}: {}", &name, why),
        Ok(_) => println!("create {} bullet.toml success!", &name),
    }
}
