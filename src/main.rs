extern crate yaml_rust;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, Read, Write};
use std::process::Command;
use yaml_rust::{Yaml, YamlLoader};

#[derive(Debug)]
struct Task {
    name: String,
    commands: Vec<String>,
}

#[derive(Debug)]
struct Variable {
    name: String,
    expression: String,
}

fn main() {
    let mut f =
        File::open("fixtures/sample.yaml").expect("Error while opening fixtures/sample.yaml");
    let mut s = String::new();
    f.read_to_string(&mut s)
        .expect("Cannot read fixtures/sample.yaml");

    let docs = YamlLoader::load_from_str(&s).expect("Cannot parse fixtures/sample.yaml");
    let doc = docs[0].as_hash().unwrap();
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
                        expression: yaml_element_as_string(value),
                    },
                );
            }
            _ => {
                let mut commands: Vec<String> = Vec::new();

                for line in value.as_vec().unwrap() {
                    commands.push(yaml_element_as_string(line));
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

fn exec_task(tasks: &HashMap<String, Task>, task_name: &String, dependencies: Vec<&String>) {
    println!("run {}", task_name);

    let task = tasks
        .get(task_name)
        .expect(&format!("Task '{}' unknown.", task_name));

    if dependencies.contains(&task_name) {
        panic!(format!(
            "Recursivity problem: '{}' get called again.",
            task_name
        ));
    }

    for command in &task.commands {
        // if command references another task, execute it
        if command.chars().next().unwrap() == '@' {
            let mut new_dependencies = dependencies.clone();
            new_dependencies.push(&task_name);

            let referenced_task_name = command.clone().split_off(1);
            println!("sub-run {}", referenced_task_name);
            exec_task(tasks, &referenced_task_name, new_dependencies);
            continue;
        }

        let mut process_command = build_process_command(command);
        let output = process_command
            .output()
            .expect(&format!("Command '{}' failed.", command));

        io::stdout().write_all(&output.stdout).unwrap();
        io::stderr().write_all(&output.stderr).unwrap();

        if !output.status.success() {
            match output.status.code() {
                Some(code) => panic!(format!("Command '{}' failed with code {}.", command, code)),
                None => panic!(format!("Command '{}' terminated by signal", command)),
            }
        }
    }
}

fn build_process_command(command: &String) -> Command {
    // let mut parts = split(command).unwrap();
    // let mut command = Command::new(parts.remove(0));

    // command.args(parts);
    // command

    let mut process_command = Command::new("sh");
    process_command.args(vec!["-c", "-e", "-u"]).arg(command);

    println!("{}", command);

    process_command
}

fn yaml_element_as_string(value: &Yaml) -> String {
    let expression: String;
    if value.as_str().is_some() {
        expression = value.clone().into_string().unwrap();
    } else if value.as_i64().is_some() {
        expression = value.clone().into_i64().unwrap().to_string();
    } else if value.as_f64().is_some() {
        expression = value.clone().into_f64().unwrap().to_string();
    } else if value.as_bool().is_some() {
        expression = value.clone().into_bool().unwrap().to_string();
    } else {
        unimplemented!();
    }

    expression
}

// todo seggregate rustake args from task args (ie. [rustake args] -- [task args])
fn get_calls_args() -> Vec<String> {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    println!("command args: {:?}", args);

    args
}
