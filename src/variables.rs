extern crate regex;
use crate::Variable;
use regex::{Match, Regex};
use std::collections::HashMap;
use std::str;

pub fn resolve(variables: &HashMap<String, Variable>) -> HashMap<String, Variable> {
    let mut parsed: HashMap<String, Variable> = HashMap::new();

    let re = Regex::new(r"(?:\$(?:\{\w+\}|\w+))").unwrap();

    

    for (name, variable) in variables {
        let mut processed_variable = variable.clone();

        if !re.is_match(&variable.value) {
            parsed.insert(name.clone(), processed_variable);
            continue;
        }

        let mut call_stack: Vec<String> = vec![name.clone()];
        for capture in re.captures_iter(&variable.value) {
            let found = capture.get(0).unwrap().as_str();
            processed_variable = process_variable(&name, processed_variable, variables, &found, &call_stack);
        }

        parsed.insert(name.clone(), processed_variable);
    }

    parsed
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
            name, call_stack.join(" -> "), call_stack
        ));
    }

    let value = &variables.get(&indexed_found).unwrap().value;

    Variable {
        name: variable.name,
        value: variable.value.replace(found, value),
    }

    // println!("{} - {:?}", name, found);
}
