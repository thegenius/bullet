use tera::GlobalFn;
use tera::Value;
use tera::Result;
use tera::Tera;
use tera::to_value;
use std::collections::HashMap;


pub fn build_tera(template_path: &str, fn_map: HashMap<&str, GlobalFn>) -> Tera {
    let mut tera: Tera = compile_templates!(format!("{}/**/*", template_path).as_str());
    for (k, v) in fn_map {
        tera.register_function(k, v);
    }
    return tera;
}