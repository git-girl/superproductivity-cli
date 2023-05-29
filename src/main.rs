use std::path::Path;
use serde_json::{Result, Value};

fn get_superprod_json(path: &Path) -> Value {
    unimplemented!();
}

fn get_superprod_data_path() -> &'static Path { 
    Path::new("~/.config/superProductivity/")
}

fn main() {
    let path = get_superprod_data_path();
    let path_as_string = &path.to_str().expect("Something went wrong...");
    println!("{}", path_as_string);
}
