use crate::Task;
use log::{debug, info};
use shellwords;
use std::collections::HashMap;
use std::mem;
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

            // remove the first char "@"
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

        run_instruction(instruction, &call_args, variables);
    }
}

fn run_instruction(
    instruction: &String,
    call_args: &Vec<String>,
    variables: &HashMap<String, String>,
) {
    let instructions_parts = shellwords::split(&instruction).unwrap();
    let (program, program_args) = instructions_parts.split_at(1);
    let program_args = expand_program_args(program_args.to_vec(), call_args, variables);
    let program = &program[0];
    let mut command = Command::new(program);

    command.args(&program_args);

    info!("{:?}", command);

    let output = command
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect(&format!("Command '{:?}' failed.", program_args));

    if !output.status.success() {
        match output.status.code() {
            Some(code) => panic!("Command '{:?}' failed with code {}.", program_args, code),
            None => panic!("Command '{:?}' terminated by signal", program_args),
        }
    }
}

fn expand_program_args(
    program_args: Vec<String>,
    call_args: &Vec<String>,
    variables: &HashMap<String, String>,
) -> Vec<String> {
    debug!("program_args: {:?}", program_args);

    let mut processed_args = program_args.clone();

    for (index, program_arg) in program_args.iter().enumerate() {
        if !variables.contains_key(program_arg) {
            debug!("  parameter unknown {}", program_arg);
            continue;
        }

        let value = variables.get(program_arg).unwrap();

        mem::replace(&mut processed_args[index], value.to_string());
        debug!("  replaced {} by {}", program_arg, value);
    }
    debug!("processed_args: {:?}", processed_args);

    if let Some(index) = processed_args.iter().position(|x| x == "$RUMAKE_ARGS") {
        let (left, right) = processed_args.split_at(index);
        let mut right = right.to_vec();
        right.remove(0);

        processed_args = left.to_vec();
        processed_args.extend_from_slice(&call_args);
        processed_args.extend_from_slice(&right);

        // panic!("-- {:?}  {:?} -- {:?} {:?}", processed_args,  index, left, right);
        debug!("  rumake args remplaced: {:?}", call_args);
    }

    processed_args
}
