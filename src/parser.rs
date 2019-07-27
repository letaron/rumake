use linked_hash_map::LinkedHashMap;
use std::fs::File;
use std::io::Read;
use yaml_rust::{Yaml, YamlLoader};

pub fn get_doc() -> LinkedHashMap<Yaml, Yaml> {
    let mut f = File::open("rumake.yaml").expect("Error while opening rumake.yaml");
    let mut s = String::new();
    f.read_to_string(&mut s).expect("Cannot read rumake.yaml");

    let docs = YamlLoader::load_from_str(&s).expect("Cannot parse rumake.yaml");
    let doc = docs[0].clone().into_hash().unwrap();

    doc
}

pub fn yaml_element_as_string(value: &Yaml) -> String {
    let string: String;
    if value.as_str().is_some() {
        string = value.clone().into_string().unwrap();
    } else if value.as_i64().is_some() {
        string = value.clone().into_i64().unwrap().to_string();
    } else if value.as_f64().is_some() {
        string = value.clone().into_f64().unwrap().to_string();
    } else if value.as_bool().is_some() {
        string = value.clone().into_bool().unwrap().to_string();
    } else {
        unimplemented!();
    }

    string
}
