@_default:
    just --list --justfile {{ justfile() }}

watch:
    @watchexec -c -r -e rs,toml cargo r

alias w := watch