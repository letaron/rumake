#/usr/bin/env bash

_rumake_completions()
{
  if [ "${#COMP_WORDS[@]}" != "2" ]; then
    return
  fi
  COMPREPLY=($(grep -E "^[a-z0-9_-.]+:" rumake.yaml | cut -d ":" -f 1 | grep -E "^${COMP_WORDS[1]}"))
}

complete -F _rumake_completions rumake
