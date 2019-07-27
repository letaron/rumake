extern crate yaml_rust;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Read;
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

    let args = get_args();
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
                    create_variable(name.clone(), as_string(value)),
                );
            }
            _ => {
                let mut commands: Vec<String> = Vec::new();

                for line in value.as_vec().unwrap() {
                    commands.push(as_string(line));
                }

                tasks.insert(name.clone(), create_task(name.clone(), commands));
            }
        }
    }

    exec_task(
        tasks
            .get(&args[0])
            .expect(&format!("Task {} unknown.", &args[0])),
    );

    println!("{:?}", tasks);
    println!("{:?}", variables);
}

fn exec_task(task: &Task) {}

fn create_task(name: String, commands: Vec<String>) -> Task {
    Task { name, commands }
}

fn create_variable(name: String, expression: String) -> Variable {
    Variable { name, expression }
}

fn as_string(value: &Yaml) -> String {
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
fn get_args() -> Vec<String> {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    println!("command args: {:?}", args);

    args
}
