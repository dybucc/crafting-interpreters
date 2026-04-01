set shell := ["fish", "-c"]
set quiet := true

alias r := run

cargo := require("cargo")

[default]
[private]
default:
    {{ just_executable() }} --list --unsorted --justfile {{ justfile() }}

# runs a command for either the given workspace member or the pwd
run command="":
    {{ cargo }} r 2> /dev/null -- {{ command }}
