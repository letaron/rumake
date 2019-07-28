# RUMAKE

`rumake` is a task runner written in Rust that tries to stay close to the sell. It doesn't attend to be a drop-in replacement for `make`.

Principal features are:
- not-coupled to a runtime.
- simple syntax (YAML).
- can pass down arguments to the instruction.
- tasks & variables dependency.
- check for infinite recursive declaration.

With arguments forwarding, no need to repeat a target for a small difference, you can factorise tasks.

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

hello: cowsay hello $from !
me:
  - "@hello"
  - echo "I\'m $USER and time is $(date)"
$from: world
```

You can use it like this:
```bash
rumake console cache:pool:prune # will run with --verbose flag
rumake console lint:twig # will run with --verbose flag

rumake encore dev --hot

rumake me
# hello world !
# I'm ... and time is ...
```

## Usage

`rumake TASK [ARGS]`

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

#### Default behavior

If the task consist of a simple intruction, CLI args are forward to the end.

With this configuration
```yaml
dkcr: docker-compose run --rm
```

will be used like that
```bash
# will run docker-compose run --rm node bash
rumake dkcr node bash
```

When the task has multiple instructions, you need to place the arguments. It allow more use cases, please see below.

#### Placing arguments

`rumake` replace the special arguments `$RUMAKE_ARGS` by CLI args.

With this configuration
```yaml
shell: docker-compose run $RUMAKE_ARGS bash 
```

will be used like that
```bash
rumake dkcr shell_debug node
```

And when referencing, it's the same principle:
```yaml
pizza: echo \"Let\'s go for a pizza with\"
# you can place the $RUMAKE_ARGS if needed
# pizza: echo \"Let\'s go for a pizza with $RUMAKE_ARGS 🍕\"
pizza_super: "@pizza super $RUMAKE_ARGS"
pizza_extra:
  - echo hmmm...
  - echo I love $RUMAKE_ARGS
  - "@pizza_super extra $RUMAKE_ARGS !"
```

You use it like this
```bash
rumake pizza cheese # Let's go for a pizza with cheese

rumake pizza_extra cheese
# hmmm...
# I love cheese
# Let's go for a pizza with super extra cheese !
```
> When referencing a task, the arguments passed to the task are the ones declared in the referencing task, not the "global" ones.

## Escaping

Notice the quote escaping
```yaml
task: echo It\\\'s a test
# or
task: echo \"It\'s a test\"

# will echo: It's a test
```

> The each task instruction is lauched by `sh -e -u -c instruction`

## Configuration

See a full working configuration [here](fixtures/example.yaml).

There is 2 types of element: `tasks` & `variables`.

### Conf file

Priority for loading (no merging is done):
1. if `rumake.yaml` exists in the working directory, it will be used.
2. if `rumake.yaml.dist` exists in the working directory, it will be used.

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
> `make` was too diverted to provide what we need but it's a building tool, not a task runner.
> We could feel that `Makefile` syntax can be tiedous to manipulate.
