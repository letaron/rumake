use linked_hash_map::LinkedHashMap;
use log::debug;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use yaml_rust::{Yaml, YamlLoader};

pub fn get_doc() -> LinkedHashMap<Yaml, Yaml> {
    let configs = vec!["rumake.yaml", "rumake.yaml.dist"];
    let mut config: Option<&str> = None;

    for _config in &configs {
        if Path::new(_config).is_file() {
            config = Some(&_config);
            debug!("configuration: {}", _config);
            break;
        }
    }

    if config.is_none() {
        panic!(
            "No config file found in working directory. Looked for {}.",
            configs.join(", ")
        );
    }

    let config = config.unwrap();

    let mut config_buffer = String::new();
    let mut file = File::open(config).expect(&format!("Error while opening {}", config));
    file.read_to_string(&mut config_buffer)
        .expect(&format!("Cannot read {}", config));

    let docs = YamlLoader::load_from_str(&config_buffer).expect("Cannot parse config");
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
