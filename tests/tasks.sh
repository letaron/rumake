#!/usr/bin/env bats

RUMAKE=target/debug/rumake

@test "echo foo" {
  run $RUMAKE echo foo
  [ "$status" -eq 0 ]
  [ "$output" = "foo" ]
}

@test "pizza" {
  run $RUMAKE pizza cheese
  [ "$status" -eq 0 ]
  [ "$output" = "Let's go for a pizza with cheese 🍕" ]
}

@test "pizza_super" {
  run $RUMAKE pizza_super sauerkraut
  [ "$status" -eq 0 ]
  [ "$output" = "Let's go for a pizza with super sauerkraut 🍕" ]
}

@test "pizza_extra" {
  run $RUMAKE pizza_extra enchilada
  [ "$status" -eq 0 ]
  [ "${lines[0]}" = "hmmm..." ]
  [ "${lines[1]}" = "I love enchilada" ]
  [ "${lines[2]}" = "Let's go for a pizza with super extra enchilada ! 🍕" ]
}

@test "fail" {
  run $RUMAKE fail
  [ "$status" -eq 101 ]
}

@test "non existent" {
  run $RUMAKE foo-$RANDOM
  [ "$status" -eq 101 ]
}

