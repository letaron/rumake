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

hello: cowsay hello
me:
  - "@hello"
  - echo "I\'m $USER and time is $(date)"
```

You can use it like this:
```bash
rumake console cache:pool:prune # will run with --verbose flag
rumake console lint:twig # will run with --verbose flag

rumake encore dev --hot

rumake me
# ...
# I'm ... and time is ...
```

## Usage

`rumake TASK [ARGS]`

### Task ordering

You can reference a taks by using `@` it's name.

> When referencing a task, the arguments passed to the task are the ones declared in the referencing task, not the "global" ones.

With this configuration
```yaml
pizza: echo Let\\\'s go for a pizza with $RUMAKE_ARGS
pizza_super: "@pizza super $RUMAKE_ARGS"
pizza_extra:
  - echo "hmmm..."
  - "@pizza_super extra $RUMAKE_ARGS !"
```
> Notice the quote escaping

You use it like this
```bash
rumake pizza cheese # Let's go for a pizza with cheese

rumake pizza_extra cheese
# hmmm...
# Let's go for a pizza with super extra cheese !
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
shell_debug:
  - echo "Current status"
  - docker-compose ps $RUMAKE_ARGS
  - "@shell --rm $RUMAKE_ARGS"

```

will be used like that
```bash
rumake dkcr shell_debug node
```

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
