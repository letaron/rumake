use crate::arguments::expand_program_args;
use crate::Task;

use log::{debug, info};
use std::collections::HashMap;
use std::process::{Command, Stdio};

/// Run the taks, recursively if needed.
pub fn exec_task(
    tasks: &HashMap<String, Task>,
    task_name: &String,
    call_args: &[String],
    command_call_stack: Vec<&String>,
    variables: &HashMap<String, String>,
) {
    debug!("run {} with {:?}", task_name, call_args);

    let task = tasks
        .get(task_name)
        .unwrap_or_else(|| panic!("Task '{}' unknown.", task_name));

    if command_call_stack.contains(&task_name) {
        panic!("Recursivity problem: '{}' get called again.", task_name);
    }

    for instruction in &task.instructions {
        let program = instruction.program.to_string();

        // if command references another task, execute it
        if program.starts_with('@') {
            let mut new_command_call_stack = command_call_stack.clone();
            new_command_call_stack.push(&task_name);

            // remove the first char "@"
            let program = program.to_string().split_off(1);

            debug!(
                "  -> run dependency {} with {:?}",
                program, instruction.arguments
            );
            let program_args =
                expand_program_args(&instruction.arguments, call_args, variables, false);

            exec_task(
                tasks,
                &program,
                &program_args,
                new_command_call_stack,
                variables,
            );
            continue;
        }

        run_instruction(
            &program,
            &instruction.arguments,
            &call_args,
            variables,
            task.instructions.len() == 1, // is_mono_insruction_task
        );
    }
}

fn run_instruction(
    program: &str,
    program_args: &[String],
    call_args: &[String],
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
        .unwrap_or_else(|_| panic!("Command '{:?}' failed.", real_command));

    if !output.status.success() {
        match output.status.code() {
            Some(code) => panic!("Command '{:?}' failed with code {}.", real_command, code),
            None => panic!("Command '{:?}' terminated by signal", real_command),
        }
    }
}
