use crate::Task;
use log::{debug, info};
use std::collections::HashMap;
use std::process::{Command, Stdio};

pub fn exec_task(
    tasks: &HashMap<String, Task>,
    task_name: &String,
    call_args: &Vec<String>,
    command_call_stack: Vec<&String>,
    variables: &HashMap<String, String>,
) {
    debug!("run {}", task_name);

    let task = tasks
        .get(task_name)
        .expect(&format!("Task '{}' unknown.", task_name));

    if command_call_stack.contains(&task_name) {
        panic!("Recursivity problem: '{}' get called again.", task_name);
    }

    for instruction in &task.instructions {
        // if command references another task, execute it
        if instruction.chars().next().unwrap() == '@' {
            let mut new_command_call_stack = command_call_stack.clone();
            new_command_call_stack.push(&task_name);

            let referenced_task_name = instruction.to_string().split_off(1);
            debug!("  -> run {}", referenced_task_name);
            exec_task(
                tasks,
                &referenced_task_name,
                &call_args,
                new_command_call_stack,
                variables,
            );
            continue;
        }

        run_instruction(task, instruction, &call_args, variables);
    }
}

fn run_instruction(
    task: &Task,
    instruction: &String,
    call_args: &Vec<String>,
    variables: &HashMap<String, String>,
) {
    let mut command = Command::new("sh");
    let instruction = expand_instruction(task, instruction, call_args, variables);

    command.args(vec!["-e", "-u", "-c"]).arg(&instruction);

    info!("{:?}", command);

    let output = command
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect(&format!("Command '{}' failed.", instruction));

    if !output.status.success() {
        match output.status.code() {
            Some(code) => panic!("Command '{}' failed with code {}.", instruction, code),
            None => panic!("Command '{}' terminated by signal", instruction),
        }
    }
}

fn expand_instruction(
    task: &Task,
    instruction: &String,
    call_args: &Vec<String>,
    variables: &HashMap<String, String>,
) -> String {
    let mut instruction = instruction.to_string();

    debug!("  original: {}", instruction);
    for (name, value) in variables {
        instruction = instruction.replace(name, value);
    }
    debug!("  replaced: {}", instruction);

    if task.instructions.len() == 1 {
        for call_arg in call_args {
            instruction = format!("{} {}", instruction, call_arg);
        }
    }

    instruction
}
