_default:
  @just --choose

checks:
  #!/usr/bin/env bash
  set -x
  cargo clippy --all-targets
  cargo fmt --all -- --check

clippy-pedantic:
  cargo clippy --workspace -- -W clippy::pedantic

examples-serve:
  cd examples_outputs && http

examples-watch:
  cd examples_outputs && npm run browser-sync

run-example name:
  bacon ex -- {{name}}

insta-review:
  cargo insta test --review
