extern crate yaml_rust;
mod parser;
mod runner;

use runner::exec_task;
use std::collections::HashMap;
use std::env;

#[derive(Debug)]
pub struct Task {
    name: String,
    commands: Vec<String>,
}

#[derive(Debug)]
pub struct Variable {
    name: String,
    expression: String,
}

fn main() {
    let doc = parser::get_doc();
    let iter = doc.iter();

    let args = get_calls_args();
    if args.len() == 0 {
        panic!("You must provide an task name!");
    }

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
                        expression: parser::yaml_element_as_string(value),
                    },
                );
            }
            _ => {
                let mut commands: Vec<String> = Vec::new();

                for line in value.as_vec().unwrap() {
                    commands.push(parser::yaml_element_as_string(line));
                }

                tasks.insert(name.clone(), Task { name, commands });
            }
        }
    }

    let task_name = &args[0];
    exec_task(&tasks, task_name, Vec::new());

    // println!("{:?}", tasks);
    // println!("{:?}", variables);
}

// todo seggregate rustake args from task args (ie. [rustake args] -- [task args])
fn get_calls_args() -> Vec<String> {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    println!("command args: {:?}", args);

    args
}
