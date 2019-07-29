use log::debug;
use std::collections::HashMap;

pub fn expand_program_args(
    program_args: &[String],
    call_args: &[String],
    variables: &HashMap<String, String>,
    is_mono_insruction_task: bool,
) -> Vec<String> {
    debug!(
        "program_args: {:?}, is_single_insruction_task: {}",
        program_args, is_mono_insruction_task
    );

    let mut processed_args = expand_variables_pass(&program_args, variables);
    debug!("expand_variables_pass: after {:?}", processed_args);

    if program_args_has_rumake_args(&processed_args) {
        processed_args = expand_rumake_args_pass(processed_args, call_args);
        debug!("expand_rumake_args_pass: after {:?}", processed_args);
    } else if is_mono_insruction_task {
        debug!("  single instruction task, forwarding: {:?}", call_args);
        processed_args.append(&mut call_args.to_vec());
        return processed_args;
    }

    debug!("  processed_args: {:?}", call_args.len());

    processed_args
}

fn expand_variables_pass(
    program_args: &[String],
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
fn expand_rumake_args_pass(args: Vec<String>, call_args: &[String]) -> Vec<String> {
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

fn program_args_has_rumake_args(args: &[String]) -> bool {
    for arg in args {
        if arg.find("$RUMAKE_ARGS").is_some() {
            return true;
        }
    }

    false
}
