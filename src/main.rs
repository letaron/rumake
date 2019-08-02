mod arguments;
mod config;
mod runner;
mod variables;

use runner::exec_task;
use shellwords;
use std::collections::HashMap;
use std::env;
use yaml_rust::Yaml;

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
    simple_logger::init().unwrap();

    let args: Vec<String> = env::args().collect();
    let task_name = get_task_name(&args);
    let calls_args = get_calls_args(&args);

    let mut tasks: HashMap<String, Task> = HashMap::new();
    let mut variables: HashMap<String, Variable> = HashMap::new();

    for (name, value) in config::get_doc().iter() {
        let name = String::from(name.as_str().unwrap());

        if name.starts_with('$') {
            variables.insert(
                name.to_string(),
                Variable {
                    name,
                    value: config::yaml_element_as_string(value),
                },
            );
        } else {
            tasks.insert(
                name.to_string(),
                Task {
                    instructions: extract_instructions(&name, &value, true),
                    name,
                },
            );
        }
    }

    let resolved_vars = variables::resolve(&variables);

    exec_task(&tasks, task_name, &calls_args, Vec::new(), &resolved_vars);
}

fn create_instruction(line: &str) -> Instruction {
    let parts = shellwords::split(line).unwrap();
    let (program, arguments) = parts.split_at(1);
    let program = &program[0];

    Instruction {
        program: program.to_string(),
        arguments: arguments.to_vec(),
    }
}

/// Extracts instructions for a taks.
/// If instruction is an array, we walk through it on only 1 depth level.
/// This is correct:
/// ```yaml
/// task:
///   - echo foo
///   - ["echo bar", "echo baz"]
/// ```
/// This is incorrect:
/// ```yaml
/// task:
///   - echo foo
///   - ["echo bar", ["echo baz", "echo goo"]]
/// ```
fn extract_instructions(name: &str, value: &Yaml, is_first_depth_level: bool) -> Vec<Instruction> {
    // early return for simple string
    if let Some(line) = value.as_str() {
        return vec![create_instruction(&line)];
    }

    let mut instructions: Vec<Instruction> = Vec::new();

    if let Some(lines) = value.as_vec() {
        for line in lines {
            if line.as_vec().is_some() && is_first_depth_level {
                instructions.append(&mut extract_instructions(&name, line, false));
            } else if let Some(line) = line.as_str() {
                instructions.push(create_instruction(line));
            } else {
                panic!(
                    "Instructions for task '{}' must be a string or an array of string.",
                    name
                );
            }
        }
    } else {
        panic!("Task '{}' must be string or an array of string.", name);
    }

    instructions
}

fn get_task_name(args: &[String]) -> &String {
    if args.len() < 2 {
        panic!("Not task provided, exiting.");
    }

    &args[1]
}

// todo seggregate rustake args from task args (ie. [rumake args] -- [task args])
fn get_calls_args(args: &Vec<String>) -> Vec<String> {
    args.clone().split_off(2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_calls_args() {
        assert_eq!(
            get_calls_args(&vec!["rumake".to_string(), "task".to_string()]).len(),
            0
        );

        let args = get_calls_args(&vec![
            "rumake".to_string(),
            "task".to_string(),
            "param".to_string(),
        ]);
        assert_eq!(args[0], "param");
        assert_eq!(args.len(), 1);
    }

    #[test]
    #[should_panic]
    fn test_get_task_name_panic() {
        get_task_name(&vec!["rumake".to_string()]);
    }

    #[test]
    fn test_get_task_name_success() {
        let args = vec!["rumake".to_string(), "task".to_string()];
        assert_eq!(get_task_name(&args), &args[1]);
    }

    #[test]
    fn test_create_instruction() {
        let instruction = create_instruction("command arg1 --arg2 value");
        assert_eq!(instruction.program, "command");
        assert_eq!(
            instruction.arguments,
            vec![
                "arg1".to_string(),
                "--arg2".to_string(),
                "value".to_string()
            ]
        );
    }
}
