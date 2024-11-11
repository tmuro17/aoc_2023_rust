set fallback

_default:
  @just --list

test:
  cargo insta test --test-runner nextest
