use crate::Task;
use std::collections::HashMap;
use std::io::{self, Write};
use std::process::Command;

pub fn exec_task(
    tasks: &HashMap<String, Task>,
    task_name: &String,
    dependencies: Vec<&String>,
    call_args: &Vec<String>,
) {
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
            exec_task(tasks, &referenced_task_name, new_dependencies, &call_args);
            continue;
        }

        runc_command(command, &call_args);
    }
}

fn runc_command(command: &String, call_args: &Vec<String>) {
    let mut process_command = Command::new("sh");
    let mut real_command = command.clone();
    for call_arg in call_args {
        real_command = format!("{} {}", command, call_arg);
    }
    process_command
        .args(vec!["-e", "-u", "-c"])
        .arg(real_command);

    println!("{:?}", process_command);

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
