@_default:
    just --list --justfile {{ justfile() }}

db name="postgres" port="5432":
    podman run \
    -e POSTGRES_USER=logback \
    -e POSTGRES_PASSWORD=password \
    -e POSTGRES_DB=logback \
    -p {{ port }}:5432 \
    --detach \
    --rm \
    --name {{ name }} \
    docker.io/postgres:17-alpine

watch:
    @watchexec -c -r -e rs,toml cargo r

alias w := watch