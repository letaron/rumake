extern crate yaml_rust;
mod parser;
mod runner;
mod variables;

use runner::exec_task;
use std::collections::HashMap;
use std::env;

#[derive(Debug)]
pub struct Task {
    name: String,
    commands: Vec<String>,
    pass_args: bool,
}

#[derive(Debug, Clone)]
pub struct Variable {
    name: String,
    value: String,
}

fn main() {
    let doc = parser::get_doc();
    let iter = doc.iter();

    let args: Vec<String> = env::args().collect();
    let task_name = get_task_name(&args);
    let calls_args = get_calls_args(&args);

    let mut tasks: HashMap<String, Task> = HashMap::new();
    let mut variables: HashMap<String, Variable> = HashMap::new();

    for (key, value) in iter {
        let name = String::from(key.as_str().unwrap());

        let first_letter = name.chars().next().unwrap();
        match first_letter {
            '$' => {
                variables.insert(
                    name.clone(),
                    Variable {
                        name,
                        value: parser::yaml_element_as_string(value),
                    },
                );
            }
            _ => {
                let mut commands: Vec<String>;
                let pass_args: bool;

                if value.as_str().is_some() {
                    commands = vec![value.clone().into_string().unwrap()];
                    pass_args = true;
                } else if value.as_vec().is_some() {
                    commands = Vec::new();
                    for line in value.as_vec().unwrap() {
                        commands.push(parser::yaml_element_as_string(line));
                    }
                    pass_args = false;
                } else {
                    panic!(format!(
                        "Task '{}' must be string or array of string.",
                        name
                    ))
                }

                tasks.insert(
                    name.clone(),
                    Task {
                        name,
                        commands,
                        pass_args,
                    },
                );
            }
        }
    }

    // exec_task(&tasks, task_name, Vec::new(), &calls_args);

    let resolved_vars = variables::resolve(&variables);
    println!("\n\n\n{:?}", resolved_vars);

    // let task_name = &args[0];
    // exec_task(&tasks, task_name, Vec::new());

    // println!("{:?}", tasks);
    // println!("{:?}", variables);
}

fn get_task_name(args: &Vec<String>) -> &String {
    if args.len() < 2 {
        panic!("You must provide an task name!");
    }

    &args[1]
}

// todo seggregate rustake args from task args (ie. [rustake args] -- [task args])
fn get_calls_args(args: &Vec<String>) -> Vec<String> {
    args.clone().split_off(2)
}
