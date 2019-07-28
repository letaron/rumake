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
    debug!("run {} with {:?}", task_name, call_args);

    let task = tasks
        .get(task_name)
        .expect(&format!("Task '{}' unknown.", task_name));

    if command_call_stack.contains(&task_name) {
        panic!("Recursivity problem: '{}' get called again.", task_name);
    }

    let is_single_insruction_task = task.instructions.len() == 1;

    for instruction in &task.instructions {
        let instructions_parts = shellwords::split(&instruction).unwrap();
        let (program, program_args) = instructions_parts.split_at(1);
        let program = &program[0];
        let program_args = program_args.to_vec();

        // if command references another task, execute it
        if program.chars().next().unwrap() == '@' {
            let mut new_command_call_stack = command_call_stack.clone();
            new_command_call_stack.push(&task_name);

            let referenced_task_name = program.to_string().split_off(1);

            debug!(
                "  -> run before {} with {:?}",
                referenced_task_name, program_args
            );
            let program_args = replace_rumake_args(program_args, call_args);

            // remove the first char "@"
            debug!("  -> run {} with {:?}", referenced_task_name, program_args);
            exec_task(
                tasks,
                &referenced_task_name,
                &program_args,
                new_command_call_stack,
                variables,
            );
            continue;
        }

        run_instruction(
            program,
            program_args,
            &call_args,
            variables,
            is_single_insruction_task,
        );
    }
}

fn run_instruction(
    program: &String,
    program_args: Vec<String>,
    call_args: &Vec<String>,
    variables: &HashMap<String, String>,
    is_single_insruction_task: bool,
) {
    let program_args = expand_program_args(
        program_args,
        call_args,
        variables,
        is_single_insruction_task,
    );

    let mut real_command = program.to_string();

    for call_arg in program_args {
        real_command = format!("{} {}", real_command, call_arg);
    }

    let mut command = Command::new("sh");

    info!("{:?}", real_command);

    let output = command
        .args(vec!["-e", "-u", "-c"])
        .arg(&real_command)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect(&format!("Command '{:?}' failed.", real_command));

    if !output.status.success() {
        match output.status.code() {
            Some(code) => panic!("Command '{:?}' failed with code {}.", real_command, code),
            None => panic!("Command '{:?}' terminated by signal", real_command),
        }
    }
}

fn expand_program_args(
    program_args: Vec<String>,
    call_args: &Vec<String>,
    variables: &HashMap<String, String>,
    is_single_insruction_task: bool,
) -> Vec<String> {
    debug!("program_args: {:?}", program_args);

    if is_single_insruction_task {
        debug!("  single instruction task, forwarding: {:?}", call_args);
        let mut programs_args = program_args.clone();
        &programs_args.append(&mut call_args.to_vec());
        return programs_args.to_vec();
    }

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

    let processed_args = replace_rumake_args(processed_args, call_args);

    debug!("  processed_args: {:?}", processed_args);

    processed_args
}

fn replace_rumake_args(program_args: Vec<String>, call_args: &Vec<String>) -> Vec<String> {
    // replace $RUMAKE_ARGS with the CLI args

    info!(
        "program_args{:?} -- call_args {:?}",
        program_args, call_args
    );

    if let Some(index) = program_args.iter().position(|x| x == "$RUMAKE_ARGS") {
        let (left, right) = program_args.split_at(index);
        let mut right = right.to_vec();
        right.remove(0);

        let mut program_args = left.to_vec();
        program_args.extend_from_slice(&call_args);
        program_args.extend_from_slice(&right);

        debug!("rumake args remplaced: {:?}", call_args);

        return program_args;
    }

    program_args
}
