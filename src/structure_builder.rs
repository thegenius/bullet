use super::tera::Context;
use super::toml;
use std::fs;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ProjectStructureItem {
    pub item_type: String,
    pub item_path: String,
    pub item_file: String,
    pub item_tmpl: String,
}

impl ProjectStructureItem {
    pub fn is_static(&self) -> bool {
        return self.item_type.eq_ignore_ascii_case("static");
    }
    pub fn is_dynamic(&self) -> bool {
        return self.item_type.eq_ignore_ascii_case("dynamic");
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectStructure {
    project_structure: Vec<ProjectStructureItem>,
}

pub fn parse_structure_item(
    item: &ProjectStructureItem,
    context: &Context,
) -> ProjectStructureItem {
    let mut tera = tera::Tera::default();
    match tera.add_raw_template("item_path", item.item_path.as_str()) {
        Err(why) => panic!(
            "parse dynamic item failed, failed to add raw template item_path: {}",
            why
        ),
        Ok(_) => (),
    }
    match tera.add_raw_template("item_file", item.item_file.as_str()) {
        Err(why) => panic!(
            "parse dynamic item failed, failed to add raw template item_file: {}",
            why
        ),
        Ok(_) => (),
    }
    match tera.add_raw_template("item_tmpl", item.item_tmpl.as_str()) {
        Err(why) => panic!(
            "parse dynamic item failed, failed to add raw template item_tmpl: {}",
            why
        ),
        Ok(_) => (),
    }
    let item_path_result = tera.render("item_path", context).unwrap();
    let item_file_result = tera.render("item_file", context).unwrap();
    let item_tmpl_result = tera.render("item_tmpl", context).unwrap();
    return ProjectStructureItem {
        item_path: item_path_result,
        item_file: item_file_result,
        item_tmpl: item_tmpl_result,
        item_type: item.item_type.clone(),
    };
}

pub fn fetch_project_structure_items(template_dir: &str) -> Vec<ProjectStructureItem> {
    let project_structure = parse_project_structure(template_dir);
    return project_structure.project_structure;
}

pub fn parse_project_structure(template_dir: &str) -> ProjectStructure {
    let structure_template_dir = format!("{}/structure/structure.toml", template_dir);
    let content = match fs::read_to_string(structure_template_dir) {
        Err(why) => panic!("failed to read structure file: {}", why),
        Ok(content) => content,
    };

    return match toml::from_str(content.as_str()) {
        Err(why) => panic!("failed to parse structure file: {}", why),
        Ok(result) => result,
    };
}
