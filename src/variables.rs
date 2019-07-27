extern crate regex;
use crate::Variable;
use regex::{Match, Regex};
use std::collections::HashMap;
use std::str;

pub fn resolve(variables: &HashMap<String, Variable>) -> HashMap<String, Variable> {
    let mut parsed: HashMap<String, Variable> = HashMap::new();
    let mut references: HashMap<&String, Vec<String>> = HashMap::new();

    let re = Regex::new(r"(?:\$(?:\{\w+\}|\w+))").unwrap();

    for (name, variable) in variables {
        let processed_variable = variable.clone();

        println!("enter {}: {:?}", name, variable);

        if !re.is_match(&variable.value) {
            println!("    no reference in {}", name);
            parsed.insert(name.clone(), processed_variable);
            println!("\n\n");
            continue;
        }

        for capture in re.captures_iter(&variable.value) {
            let found = String::from(capture.get(0).unwrap().as_str());
            if !variables.contains_key(&found) {
                println!("    unknow reference {}", found);
                continue;
            }

            if !references.contains_key(name) {
                let mut refrenceds = Vec::new();
                refrenceds.push(found);
                references.insert(&name, refrenceds);
            } else {
                let refrenceds = references.get_mut(name).unwrap();
                refrenceds.push(found);
            }
        }

        println!("\n\n");
    }

    println!("\n\n{:#?}", parsed);
    println!("\n\n{:#?}", references);

    for name in references.keys() {
        check_cyclic_dependencies(
            &name,
            &name,
            references.get(name).unwrap(),
            &references,
            true,
        );
    }

    parsed
}

fn check_cyclic_dependencies(
    checked: &String,
    original: &String,
    referenceds: &Vec<String>,
    references: &HashMap<&String, Vec<String>>,
    first_call: bool,
) {
    println!(
        "check_cyclic_dependencies {} / {:?} / {:?}",
        checked, original, referenceds
    );

    if referenceds.contains(original) {
        panic!("Cyclic dependency found for variable {}", original);
    }

    for referenced in referenceds {
        println!("       check {}", referenced);

        if references.contains_key(&referenced) {
            println!("       -> on recheck {}", referenced);
            check_cyclic_dependencies(
                referenced,
                original,
                references.get(referenced).unwrap(),
                references,
                false,
            );
        } else {
            println!("       c bon {}", referenced);
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

    println!("on process {}, on a déjà {:?}", name, call_stack);

    if !variables.contains_key(&indexed_found) {
        println!("var: {} - no internal variable for {}", name, found);
        return variable;
    }

    println!("var: {} - replace {}", name, found);

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

    // println!("{} - {:?}", name, found);
}
