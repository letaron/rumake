mod arguments;
mod config;
mod runner;
mod variables;

use runner::exec_task;
use shellwords;
use std::collections::HashMap;
use std::env;

#[derive(Debug)]
pub struct Instruction {
    program: String,
    arguments: Vec<String>,
}

#[derive(Debug)]
pub struct Task {
    name: String,
    instructions: Vec<Instruction>,
}

#[derive(Debug, Clone)]
pub struct Variable {
    name: String,
    value: String,
}

fn main() {
    pretty_env_logger::init();

    let doc = config::get_doc();
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
                    name.to_string(),
                    Variable {
                        name,
                        value: config::yaml_element_as_string(value),
                    },
                );
            }
            _ => {
                let mut instructions: Vec<Instruction> = Vec::new();

                if let Some(line) = value.as_str() {
                    let parts = shellwords::split(line).unwrap();
                    let (program, arguments) = parts.split_at(1);
                    let program = &program[0];
                    instructions = vec![Instruction {
                        program: program.to_string(),
                        arguments: arguments.to_vec(),
                    }];
                } else if let Some(lines) = value.as_vec() {
                    for line in lines {
                        let parts = shellwords::split(line.as_str().unwrap()).unwrap();
                        let (program, arguments) = parts.split_at(1);
                        let program = &program[0];
                        instructions.push(Instruction {
                            program: program.to_string(),
                            arguments: arguments.to_vec(),
                        });
                    }
                } else {
                    panic!("Task '{}' must be string or array of string.", name)
                }

                tasks.insert(name.to_string(), Task { name, instructions });
            }
        }
    }

    let resolved_vars = variables::resolve(&variables);

    exec_task(&tasks, task_name, &calls_args, Vec::new(), &resolved_vars);
}

fn get_task_name(args: &[String]) -> &String {
    if args.len() < 2 {
        panic!("You must provide an task name!");
    }

    &args[1]
}

// todo seggregate rustake args from task args (ie. [rumake args] -- [task args])
fn get_calls_args(args: &Vec<String>) -> Vec<String> {
    args.clone().split_off(2)
}
