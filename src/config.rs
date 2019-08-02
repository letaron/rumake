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
    let mut file = File::open(config).unwrap_or_else(|_| panic!("Error while opening {}", config));
    file.read_to_string(&mut config_buffer)
        .unwrap_or_else(|_| panic!("Cannot read {}", config));

    let docs = YamlLoader::load_from_str(&config_buffer).expect("Cannot parse config");

    docs[0].clone().into_hash().unwrap()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_yaml_element_as_string_string() {
        let docs = YamlLoader::load_from_str("foo").unwrap();
        assert_eq!(yaml_element_as_string(&docs[0]), "foo");
    }

    #[test]
    fn test_yaml_element_as_string_integer() {
        let docs = YamlLoader::load_from_str("123").unwrap();
        assert_eq!(yaml_element_as_string(&docs[0]), "123");
    }

    #[test]
    fn test_yaml_element_as_string_float() {
        let docs = YamlLoader::load_from_str("13.37").unwrap();
        assert_eq!(yaml_element_as_string(&docs[0]), "13.37");
    }

    #[test]
    fn test_yaml_element_as_string_bool() {
        let docs = YamlLoader::load_from_str("true").unwrap();
        assert_eq!(yaml_element_as_string(&docs[0]), "true");
    }

    #[test]
    #[should_panic]
    fn test_yaml_element_as_string_null_panic() {
        let s = "~";
        let docs = YamlLoader::load_from_str(s).unwrap();
        yaml_element_as_string(&docs[0]);
    }
}
