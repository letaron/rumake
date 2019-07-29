#!/usr/bin/env bats

@test "echo foo" {
  run rumake echo foo
  [ "$status" -eq 0 ]
  [ "$output" = "foo" ]
}

@test "pizza" {
  run rumake pizza cheese
  [ "$status" -eq 0 ]
  [ "$output" = "Let's go for a pizza with cheese 🍕" ]
}

@test "pizza_super" {
  run rumake pizza_super sauerkraut
  [ "$status" -eq 0 ]
  [ "$output" = "Let's go for a pizza with super sauerkraut 🍕" ]
}

@test "pizza_extra" {
  run rumake pizza_extra enchilada
  [ "$status" -eq 0 ]
  [ "${lines[0]}" = "hmmm..." ]
  [ "${lines[1]}" = "I love enchilada" ]
  [ "${lines[2]}" = "Let's go for a pizza with super extra enchilada ! 🍕" ]
}

@test "fail" {
  run rumake fail
  [ "$status" -eq 101 ]
}

@test "non existent" {
  run rumake foo-$RANDOM
  [ "$status" -eq 101 ]
}

