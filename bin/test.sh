#!/bin/bash -eu

get_tasks() {
    grep -E "^[a-z0-9_-.]+:" rumake.yaml | cut -d ":" -f 1 | grep -v -E ".*$1.*"
}

# test w/o parameter
EXCLUDE="fail"
TASKS=$(get_tasks $EXCLUDE)

for task in $TASKS
do
    cargo run $task
done

EXCLUDE="ls|cargo|ps|fail"
TASKS=$(get_tasks $EXCLUDE)

# tests with a parameter
for task in $TASKS
do
    cargo run $task foo
done

# failure is what we want
set +e

fail() {
    $@
    if [[ "$?" != 101 ]]; then exit 1; fi
}

fail cargo run fail
fail cargo run non_existent
