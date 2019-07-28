# RUMAKE

`rumake` is a task runner written in Rust that tries to stay close to the sell. It doesn't attend to be a drop-in replacement for `make`.

## Installation

```shell
cargo install --git https://github.com/letaron/rumake.git
```

## Example

Let's say you have this configuration:
```yaml
console: docker-compose run --rm php bin/console $console_args
$console_args: --verbose

encore: docker-compose run --rm node yarn encore

me: echo $USER
```

You can use it like this:
```bash
rumake console cache:pool:prune
rumake console lint:twig

rumake encore dev --hot

rumake me # echo the user
```

## Usage

`rumake TASK [ARGS]`


## Features

- simple syntax (YAML).
- can pass down arguments to the instruction.
- can reference tasks & variables.
- check for recursivity.

With arguments forwarding, no need to repeat a target for a small difference, you can factorise tasks.

## Configuration

Priority:
1. if `rumake.yaml` exists in the working directory, it will be used.
2. if `rumake.yaml.dist` exists in the working directory, it will be used.

There is 2 types of element: `tasks` & `variables`.

### Tasks

- Task name is any value not begining with a `$`.
- Task contains either an array of string or a string (the instruction).
- In case of task is only a string, CLI args are forwarded.
- A task can reference another task by prefixing it's name with `@`.

```yaml
task1: instruction1

task2:
  - instruction1
  - instruction2
  - for file in $(ls); do
      echo $file;
    done
  # ...
```

#### Reference tasks

You need to write the command name prefixed with `@`

```yaml
task1: echo "foo"
task2:
  - "@cmd1"
  - echo "bar"
  - "@cmd1"
```

usage
```bash
rumake task2
# foo
# bar
# foo
```

### Variables

 - Variables name starts with `$`.
 - Variable can reference other variable with their name.

```yaml
$foo: foo
$bar: bar baz${foo}51 # computes to "bar bazfoo51"
```

## Why ?

We needed a tool close to the OS and not needing a specific language (Python, PHP, Node, ...).

Being not writed in a interpreted langage allows us to be free from a runtime & ease the interface with other tools.

> Why not using `make` ? \
> `make` was too diverted to provide what we need but it's a building tool, not a task runner. \
> `Makefile` syntax can be tiedous to manipulate.
