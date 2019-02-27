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
        Ok(_) => ()
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

pub fn install_template_from_git(name: String, url: String) {
    if name.is_empty() || url.is_empty() {
        panic!("install name is empty or url is empty!")
    }
    let template_path = gen_template_path(&name);
    println!("template install path: {}", &template_path.display());
    match Repository::clone(url.as_str(), &template_path) {
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