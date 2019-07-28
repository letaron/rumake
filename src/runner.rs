use crate::Task;
use log::{debug, info};
use shellwords;
use std::collections::HashMap;
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
                "  -> run dependency {} with {:?}",
                referenced_task_name, program_args
            );
            let program_args = expand_program_args(&program_args, call_args, variables, false);
            info!("program_args after: {:?}", program_args);

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
            &program_args,
            &call_args,
            variables,
            is_single_insruction_task,
        );
    }
}

fn run_instruction(
    program: &String,
    program_args: &Vec<String>,
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

fn expand_variables_pass(
    program_args: &Vec<String>,
    variables: &HashMap<String, String>,
) -> Vec<String> {
    let mut processed_args: Vec<String> = Vec::new();
    // info!("expand_variables_pass: on recoit {:?}", args);

    for program_arg in program_args {
        if !variables.contains_key(program_arg) {
            debug!("  parameter unknown {}", program_arg);
            processed_args.push(program_arg.to_string());
            continue;
        }

        let value = variables.get(program_arg).unwrap();
        processed_args.push(value.to_string());
        debug!("  replaced {} by {}", program_arg, value);
    }

    // info!("expand_variables_pass: on retourne {:?}", processed_args);

    processed_args
}

/// $RUMAKE_ARGS pass
/// We can't look only for $RUMAKE_ARGS because of:
/// ```yaml
/// task: echo $RUMAKE_ARGS toto # two args: ["$RUMAKE_ARGS", "toto"]
/// is different from
/// task: echo "$RUMAKE_ARGS toto" # one arg: "$RUMAKE_ARGS toto"
/// ```
fn expand_rumake_args_pass(args: Vec<String>, call_args: &Vec<String>) -> Vec<String> {
    let mut processed_args: Vec<String> = Vec::new();
    let flattened_call_args = &call_args.join(" ");

    // info!("expand_rumake_args_pass: on recoit {:?}", args);

    for value in args {
        debug!("    in {}", value);
        // normalize to $RUMAKE_ARGS before replacing
        let value = value
            .replace("${RUMAKE_ARGS}", "$RUMAKE_ARGS")
            .replace("$RUMAKE_ARGS", flattened_call_args);
        debug!("      -> {}", value);

        processed_args.push(value)
    }

    // info!("expand_rumake_args_pass: on retourne {:?}", processed_args);

    processed_args
}

fn program_args_has_rumake_args(args: &Vec<String>) -> bool {
    for arg in args {
        if let Some(_) = arg.find("$RUMAKE_ARGS") {
            return true;
        }
    }

    false
}

fn expand_program_args(
    program_args: &Vec<String>,
    call_args: &Vec<String>,
    variables: &HashMap<String, String>,
    is_single_insruction_task: bool,
) -> Vec<String> {
    debug!(
        "program_args: {:?}, is_single_insruction_task: {}",
        program_args, is_single_insruction_task
    );

    let mut processed_args = expand_variables_pass(&program_args, variables);
    debug!("expand_variables_pass: after {:?}", processed_args);

    if program_args_has_rumake_args(&processed_args) {
        processed_args = expand_rumake_args_pass(processed_args, call_args);
        debug!("expand_rumake_args_pass: after {:?}", processed_args);
    } else {
        if is_single_insruction_task {
            debug!("  single instruction task, forwarding: {:?}", call_args);
            &processed_args.append(&mut call_args.to_vec());
            return processed_args;
        }
    }

    debug!("  processed_args: {:?}", call_args.len());

    processed_args
}
