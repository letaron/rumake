extern crate log;
extern crate regex;

use crate::Variable;
use log::{debug, info, warn};
use regex::{Match, Regex};
use std::collections::HashMap;
use std::str;

pub fn resolve(variables: &HashMap<String, Variable>) -> HashMap<String, Variable> {
    let mut parsed: HashMap<String, Variable> = HashMap::new();
    let references = get_references(variables);

    for name in references.keys() {
        check_cyclic_dependencies(&name, &name, references.get(name).unwrap(), &references);
    }

    parsed
}

fn get_references(variables: &HashMap<String, Variable>) -> HashMap<&String, Vec<String>> {
    let mut references: HashMap<&String, Vec<String>> = HashMap::new();

    let re = Regex::new(r"(?:\$(?:\{\w+\}|\w+))").unwrap();

    for (name, variable) in variables {
        debug!("reference check: {} - {:?}", name, variable);

        if !re.is_match(&variable.value) {
            debug!("  no reference in {}\n", name);
            continue;
        }

        for capture in re.captures_iter(&variable.value) {
            let found = String::from(capture.get(0).unwrap().as_str())
                .replace('{', "")
                .replace('}', "");
            if !variables.contains_key(&found) {
                debug!("  unknow reference {}", found);
                continue;
            }

            debug!("  add {}", found);
            if !references.contains_key(name) {
                let mut refrenceds = Vec::new();
                refrenceds.push(found);
                references.insert(&name, refrenceds);
            } else {
                let refrenceds = references.get_mut(name).unwrap();
                refrenceds.push(found);
            }
        }

        debug!("\n");
    }

    references
}

fn check_cyclic_dependencies(
    checked: &String,
    original: &String,
    referenceds: &Vec<String>,
    references: &HashMap<&String, Vec<String>>,
) {
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
            check_cyclic_dependencies(
                referenced,
                original,
                references.get(referenced).unwrap(),
                references,
            );
        } else {
            debug!("    valid {}", referenced);
        }
    }
}

fn process_variable(
    name: &String,
    variable: Variable,
    variables: &HashMap<String, Variable>,
    found: &str,
    call_stack: &Vec<String>,
) -> Variable {
    let indexed_found = String::from(found).replace('{', "").replace('}', "");

    debug!("on process {}, on a déjà {:?}", name, call_stack);

    if !variables.contains_key(&indexed_found) {
        debug!("var: {} - no internal variable for {}", name, found);
        return variable;
    }

    debug!("var: {} - replace {}", name, found);

    let next_call = String::from(found);
    if call_stack.contains(&next_call) {
        panic!(format!(
            "var: recursivity problem: '{}' get referenced again ({}). {:?}",
            name,
            call_stack.join(" -> "),
            call_stack
        ));
    }

    let value = &variables.get(&indexed_found).unwrap().value;

    Variable {
        name: variable.name,
        value: variable.value.replace(found, value),
    }

    // debug!("{} - {:?}", name, found);
}
