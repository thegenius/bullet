// use std::collections::HashMap;
// use tera::GlobalFn;
use tera::Tera;

pub fn build_tera(template_path: &str) -> Tera {
    return compile_templates!(format!("{}/**/*", template_path).as_str());
}

// pub fn build_tera_witch_fn_map(template_path: &str, fn_map: HashMap<&str, GlobalFn>) -> Tera {
//     let mut tera: Tera = compile_templates!(format!("{}/**/*", template_path).as_str());
//     for (k, v) in fn_map {
//         tera.register_function(k, v);
//     }
//     return tera;
// }
