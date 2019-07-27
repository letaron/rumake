use crate::Task;
use log::{debug, info};
use std::collections::HashMap;
use std::io::{self, Write};
use std::process::{Command, Stdio};

pub fn exec_task(
    tasks: &HashMap<String, Task>,
    task_name: &String,
    call_args: &Vec<String>,
    command_call_stack: Vec<&String>,
    variables: &HashMap<String, String>,
) {
    println!("run {}", task_name);

    let task = tasks
        .get(task_name)
        .expect(&format!("Task '{}' unknown.", task_name));

    if command_call_stack.contains(&task_name) {
        panic!(format!(
            "Recursivity problem: '{}' get called again.",
            task_name
        ));
    }

    for command in &task.commands {
        // if command references another task, execute it
        if command.chars().next().unwrap() == '@' {
            let mut new_command_call_stack = command_call_stack.clone();
            new_command_call_stack.push(&task_name);

            let referenced_task_name = command.clone().split_off(1);
            println!("sub-run {}", referenced_task_name);
            exec_task(
                tasks,
                &referenced_task_name,
                &call_args,
                new_command_call_stack,
                variables,
            );
            continue;
        }

        run_command(command, &call_args, variables);
    }
}

fn run_command(command: &String, call_args: &Vec<String>, variables: &HashMap<String, String>) {
    let mut process_command = Command::new("sh");
    let mut real_command = command.clone();

    info!("original command {}", real_command);
    for (name, value) in variables {
        real_command = real_command.replace(name, value);
    }
    info!("cleaned command {}", real_command);

    for call_arg in call_args {
        real_command = format!("{} {}", real_command, call_arg);
    }
    process_command
        .args(vec!["-e", "-u", "-c"])
        .arg(real_command);

    println!("{:?}", process_command);

    let output = process_command
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect(&format!("Command '{}' failed.", command));

    if !output.status.success() {
        match output.status.code() {
            Some(code) => panic!(format!("Command '{}' failed with code {}.", command, code)),
            None => panic!(format!("Command '{}' terminated by signal", command)),
        }
    }
}
