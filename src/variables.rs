extern crate regex;
use crate::Variable;
use regex::{Match, Regex};
use std::collections::HashMap;
use std::str;

pub fn resolve(variables: &HashMap<String, Variable>) -> HashMap<String, Variable> {
    let mut parsed: HashMap<String, Variable> = HashMap::new();

    let re = Regex::new(r"(?:\$(?:\{\w+\}|\w+))").unwrap();

    let mut call_stack: Vec<String> = Vec::new();

    for (name, variable) in variables {
        if !re.is_match(&variable.value) {
            parsed.insert(name.clone(), variable.clone());
            continue;
        }

        for cap in re.captures_iter(&variable.value) {
            // println!("{} - {:?}", name, cap);
            let found = cap.get(0).unwrap().as_str();
            call_stack.push(String::from(found));
            replace(&name, variables, &found, &call_stack);
        }
    }

    parsed
}

fn replace(
    name: &String,
    variables: &HashMap<String, Variable>,
    found: &str,
    call_stack: &Vec<String>,
) {
    let indexed_found = String::from(found).replace('{', "").replace('}', "");

    if !variables.contains_key(&indexed_found) {
        println!(
            "{} on zappe pour {} avec {} - {:?}",
            indexed_found, name, found, call_stack
        );
    }

    println!(
        "on remplace pour {} avec {} - {:?}",
        name, found, call_stack
    );

    if call_stack.contains(&name) {
        panic!(format!(
            "Recursivity problem: '{}' get referenced again.",
            name
        ));
    }

    // println!("{} - {:?}", name, found);
}
