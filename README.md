# RUMAKE

[![Build Status](https://travis-ci.org/letaron/rumake.svg?branch=master)](https://travis-ci.org/letaron/rumake)

`rumake` is a task runner written in Rust that tries to stay close to the shell. It doesn't attend to be a drop-in replacement for `make`.

Principal features are:
- not-coupled to a runtime.
- simple syntax (YAML).
- can pass down arguments to the instruction.
- tasks & variables dependency.
- check for infinite recursive declaration.

## Installation

```shell
cargo install --git https://github.com/letaron/rumake.git
```

## Quick start

This tool make use of `tasks` & `variables`. You run tasks that can use variales.

Let's say you have this configuration:
```yaml
# rumake.yaml
dkcr: docker-compose run --rm

hello: "echo 🐄: hello $from !"
me:
  - "@hello"
  - echo "I\'m $USER and time is $(date)"
$from: world
```

You can use it like this:
```bash
rumake dkcr node bash # docker-compose run --rm node bash

rumake me
# 🐄: hello world !
# I'm ... and time is ...
```

## Usage

Calling `rumake` is pretty straight-forward:
```bash
rumake TASK [ARGS]
```

`ARGS` will be used in task's instruction.

### Task referencing

You can reference a taks by `@`-name.

```yaml
task1: echo "baz"
task2:
  - echo "foo"
  - "@task1"
```

usage
```bash
rumake task2
# foo
# baz
```

### Passing arguments

You can pass down arguments from the CLI to the task's instruction and choose wich instruction will receive it, and where.
With arguments forwarding, no need to repeat a target for a small difference, you can factorise tasks.


#### Default behavior

If the task consist of a simple intruction, CLI args are appened to the end.

With this configuration:
```yaml
dkcr: docker-compose run --rm
```

will be used like that:
```bash
rumake dkcr node bash # will run docker-compose run --rm node bash
```

When the task has multiple instructions, you need to place the arguments. It allow more use cases, please see below.

#### Placing arguments

`rumake` replace the special arguments `$RUMAKE_ARGS` by CLI args.

With this configuration:
```yaml
shell: docker-compose run $RUMAKE_ARGS bash
```

will be used like that:
```bash
rumake dkcr shell node # will run docker-compose run node bash
```

And when referencing, it's the same principle:
```yaml
pizza: echo \"Let\'s go for a pizza with $RUMAKE_ARGS 🍕\"
pizza_super: "@pizza super $RUMAKE_ARGS"
pizza_extra:
  - ["echo hmmm...", "echo I love $RUMAKE_ARGS"]
  - "@pizza_super extra $RUMAKE_ARGS !"
```

You use it like this
```bash
rumake pizza cheese # will output: Let's go for a pizza with cheese ! 🍕

rumake pizza_extra cheese # will output:
# hmmm...
# I love cheese
# Let's go for a pizza with super extra cheese ! 🍕
```
> When referencing a task, the arguments passed to the task are the ones declared in the referencing task, like a new direct call, not the "global" ones.

## Escaping

Notice the quote escaping. If an instruction need a quote, it needs to get out from YAML first.
```yaml
task: echo It\\\'s a test # after YAML parsing: It\'s a test
# or
task: echo \"It\'s a test\" # after YAML parsing: "It's a test"

# will echo: It's a test
```

> The each task instruction is lauched by `sh -e -u -c ...`.

## Configuration

See a full working configuration [here](fixtures/example.yaml).

### Config file used

Priority for loading (no merging is done):
1. if `rumake.yaml` exists in the working directory, it will be used.
2. if `rumake.yaml.dist` exists in the working directory, it will be used.

### Task definition

- Task name is any value not begining with a `$`.
- Task are made of instruction(s).
- Instructionss are either an array of string or a string.
- In case of instruction is only a string, CLI args are forwarded.
- A task can reference another task by prefixing it's name with `@`.

```yaml
task1: instruction1

task2:
  - instruction1
  - instruction2 $foo
  - ["@task1 with param", "instruction3"]
  - for file in $(ls); do
      echo $file;
    done
  # ...
```

### Variable definition

 - Variables name starts with `$`.
 - Variable can reference other variable with their name.

```yaml
$foo: foo
$bar: bar baz${foo}51 # computes to "bar bazfoo51"
```

## Shell completion

Shell completion is supported for **Bash**, move [`fixtures/rumake-completion.bash`] to `$XDG_CONFIG_HOME/bash_completion` or `/etc/bash_completion.d/`, ie.
```bash
cp fixtures/rumake-completion.bash ${XDG_CONFIG_HOME:-/etc/bash_completion.d/}/rumake
```

## Why ?

We needed a tool close to the OS and not needing a specific language (Python, PHP, Node, ...).

Being not writed in a interpreted langage allows us to be free from a runtime & ease the interface with other tools.

> Why not using `make` ? \
> `make` was too diverted to provide what we need but it's a building tool, not a task runner.
> We could feel that `Makefile` syntax can be tiedous to manipulate.

### See also

- [cargo-make](https://github.com/sagiegurari/cargo-make)
