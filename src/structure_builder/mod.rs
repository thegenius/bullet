use config;
use config::Value;
use config::Source;
use config::ConfigError;
use walkdir::WalkDir;
use std::fs::metadata;
use std::collections::HashMap;

//#[derive(Debug, PartialEq, Serialize, Deserialize)]
//struct ProjectStructureItem<'a> {
//    path: &'a str,
//    name: &'a str,
//    template: &'a str
//}
//
//#[derive(Debug, PartialEq, Serialize, Deserialize)]
//struct ProjectStructure<'a> {
//    item: &'a Vec<ProjectStructureItem<'a>>
//}

pub fn process_structure(structure_file: &str) {
    let mut config = config::Config::default();
    config.merge(config::File::with_name(structure_file));
    let config_map = config.try_into::<HashMap<String, Vec<Vec<String>>>>().unwrap();
    let project_structure = config_map.get("project_structure").unwrap();
//    dbg!(project_structure);
    for structure_item in project_structure {
        assert_eq!(structure_item.len(), 3);
        let template_dir = structure_item.get(0).unwrap();
        println!("path: {}", template_dir);
        let dir_md = metadata(template_dir).unwrap();
        println!("{}", dir_md.is_file());
//        dbg!(&dir_md);



        println!("name: {}", structure_item.get(1).unwrap());
        println!("template: {}", structure_item.get(2).unwrap());
    }
//    let structure_array = project_structure.try_into::<Vec<Value>>();
//    let structure_array = (&project_structure).into_array().unwrap();
//
//    for structure_item in structure_array{
//        println!("{}", structure_item);
//    }
}