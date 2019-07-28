# RUMAKE

`rumake` is a task runner written in Rust.

## Why ?

We needed a tool close to the OS and not needing a specific language (Python, PHP, Node, ...).

Being not writed in a interpreted langage allows us to be free from a runtime & ease the interface with other tools.

> Why not using `make` ?
> `make` was too diverted to provide what we need but it's a building tool, not a task runner.

## Benefits

- simple syntax (YAML)
- can pass down arguments to the task

With arguments forwarding, no need to repeat a target for a small difference, you can factorise tasks.

## Usage

`rumake TASK [ARGS]`

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

## Configuration

Priority:
1. if `rumake.yaml` exists in the working directory, it will be used.
2. if `rumake.yaml.dist` exists in the working directory, it will be used.

There is 2 types of element: `commands` & `variables`.

### Commands

- Command name is any value not begining with a `$`.
- Instruction is either an array of string or a string.
- In case of the instructions are just a string, CLI args are forwarded.
- Command can reference another command by prefixing it's name with "@".

```yaml
cmd1: instruction1

cmd2:
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
$bar: bar baz${foo}51 # computes to "bar baefoo51"
```


