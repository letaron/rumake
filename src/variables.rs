use crate::Variable;
use log::debug;
use regex::Regex;
use std::collections::HashMap;

pub fn resolve(variables: &HashMap<String, Variable>) -> HashMap<String, String> {
    let mut values: HashMap<String, String> = HashMap::new();
    let references = get_references(variables);

    for name in references.keys() {
        check_cyclic_dependencies(&name, &name, &references);
    }

    for name in variables.keys() {
        values.insert(
            name.to_string(),
            compute_value(&name, variables, &references),
        );
    }

    values
}

fn format_regex_match(name: String) -> String {
    name.replace('{', "").replace('}', "")
}

fn compute_value(
    name: &String,
    variables: &HashMap<String, Variable>,
    references: &HashMap<&String, Vec<String>>,
) -> String {
    debug!("compute value for variable {}", name);

    let value = variables.get(name).unwrap().value.to_string();

    if !references.contains_key(name) {
        debug!("  found in variables {}", value);
        return value;
    }

    let mut value = value;
    for referenced in references.get(name).unwrap() {
        let referenced_value =
            String::from(compute_value(referenced, variables, references).as_str());
        debug!("  replace {} by {}", referenced, referenced_value);
        value = value.replace(referenced, &referenced_value)
    }

    value
}

/// Extract all the variables referencing another variable and theirs referenceds variables.
/// It's a simple `[String: String[]]`.
fn get_references(variables: &HashMap<String, Variable>) -> HashMap<&String, Vec<String>> {
    let mut references: HashMap<&String, Vec<String>> = HashMap::new();
    let re = Regex::new(r"(?:\$(?:\{\w+\}|\w+))").unwrap();

    for (name, variable) in variables {
        debug!("reference check: {} - {:?}", name, variable);

        // if there is no reference, don't register
        if !re.is_match(&variable.value) {
            debug!("  {} has no references", name);
            continue;
        }

        for capture in re.captures_iter(&variable.value) {
            let referenced = format_regex_match(String::from(capture.get(0).unwrap().as_str()));

            // it's not a variable we know
            if !variables.contains_key(&referenced) {
                debug!("  unknow reference {}", referenced);
                continue;
            }

            debug!("  add {}", referenced);
            // add (create the vector if needed)
            if !references.contains_key(name) {
                let mut refrenceds = Vec::new();
                refrenceds.push(referenced);
                references.insert(&name, refrenceds);
            } else {
                let refrenceds = references.get_mut(name).unwrap();
                refrenceds.push(referenced);
            }
        }
    }

    references
}

fn check_cyclic_dependencies(
    checked: &String,
    original: &String,
    references: &HashMap<&String, Vec<String>>,
) {
    let referenceds = references.get(checked).unwrap();

    debug!(
        "check_cyclic_dependencies {} / {:?} / {:?}",
        checked, original, referenceds
    );

    if referenceds.contains(original) {
        panic!("Cyclic dependency found for variable {}", original);
    }

    for referenced in referenceds {
        debug!("  check {}", referenced);

        if references.contains_key(&referenced) {
            debug!("    -> check_cyclic_dependencies {}", referenced);
            check_cyclic_dependencies(referenced, original, references);
        } else {
            debug!("    valid {}", referenced);
        }
    }
}
