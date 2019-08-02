use log::debug;
use std::collections::HashMap;

pub fn expand_program_args(
    program_args: &[String],
    call_args: &[String],
    variables: &HashMap<String, String>,
    is_mono_insruction_task: bool,
) -> Vec<String> {
    debug!(
        "expand_program_args: {:?}, is_single_insruction_task: {}",
        program_args, is_mono_insruction_task
    );

    let mut processed_args = replace_known_variables(&program_args, variables);

    if program_args_has_rumake_args(&processed_args) {
        debug!("  rumake args found");
        processed_args = replace_rumake_args(processed_args, call_args);
    } else if is_mono_insruction_task {
        debug!("  single instruction task, forwarding: {:?}", call_args);
        processed_args.append(&mut call_args.to_vec());
        return processed_args;
    }

    debug!("  processed_args: {:?}", processed_args);

    processed_args
}

fn replace_known_variables(
    program_args: &[String],
    variables: &HashMap<String, String>,
) -> Vec<String> {
    debug!("  replace_known_variables");

    let mut processed_args: Vec<String> = Vec::new();

    for program_arg in program_args {
        if !variables.contains_key(program_arg) {
            debug!("    parameter unknown {}", program_arg);
            processed_args.push(program_arg.to_string());
            continue;
        }

        let value = variables.get(program_arg).unwrap();
        processed_args.push(value.to_string());
        debug!("    replaced {} by {}", program_arg, value);
    }

    processed_args
}

/// $RUMAKE_ARGS pass
/// We can't look only for $RUMAKE_ARGS because of:
/// ```yaml
/// task: echo $RUMAKE_ARGS toto # two args: ["$RUMAKE_ARGS", "toto"]
/// is different from
/// task: echo "$RUMAKE_ARGS toto" # one arg: "$RUMAKE_ARGS toto"
/// ```
fn replace_rumake_args(args: Vec<String>, call_args: &[String]) -> Vec<String> {
    let mut processed_args: Vec<String> = Vec::new();
    let flattened_call_args = &call_args.join(" ");

    for value in args {
        // normalize to $RUMAKE_ARGS before replacing
        let replaced = normalize_rumake_args(&value).replace("$RUMAKE_ARGS", flattened_call_args);
        debug!("    {} -> {}", value, replaced);

        processed_args.push(replaced)
    }

    processed_args
}

fn normalize_rumake_args(raw: &String) -> String {
    raw.replace("${RUMAKE_ARGS}", "$RUMAKE_ARGS")
}

fn program_args_has_rumake_args(args: &[String]) -> bool {
    for arg in args {
        if normalize_rumake_args(&arg).find("$RUMAKE_ARGS").is_some() {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_rumake_args() {
        assert_eq!(
            normalize_rumake_args(&"foo bar".to_string()),
            "foo bar".to_string()
        );
        assert_eq!(
            normalize_rumake_args(&"bar ${RUMAKE_ARGS} baz".to_string()),
            "bar $RUMAKE_ARGS baz".to_string()
        );
        assert_eq!(
            normalize_rumake_args(&"bar $RUMAKE_ARGSbaz".to_string()),
            "bar $RUMAKE_ARGSbaz".to_string()
        );
        assert_eq!(
            normalize_rumake_args(&"bar ${RUMAKE_ARGS}baz".to_string()),
            "bar $RUMAKE_ARGSbaz".to_string()
        );
    }

    #[test]
    fn program_args_has_rumake_args_true() {
        assert_eq!(
            program_args_has_rumake_args(&["foo".to_string(), "bar $RUMAKE_ARGS baz".to_string()]),
            true
        );
        assert_eq!(
            program_args_has_rumake_args(&[
                "foo".to_string(),
                "bar ${RUMAKE_ARGS} baz".to_string()
            ]),
            true
        );
        assert_eq!(
            program_args_has_rumake_args(&["foo".to_string(), "bar${RUMAKE_ARGS}baz".to_string()]),
            true
        );
    }

    #[test]
    fn program_args_has_rumake_args_false() {
        assert_eq!(
            program_args_has_rumake_args(&["foo".to_string(), "bar".to_string()]),
            false
        );
    }
}
