# Contributing

## Run against kdl-rs

```shell
cargo build -p kdl-rs-decoder
cargo run -- --decoder target/debug/kdl-rs-decoder
```

## Pre-commit hooks

To set up pre-commit hooks:

1. Install [hooky](https://github.com/brandonchinn178/hooky)
2. `hooky install`

## Regenerate expected JSON files

```shell
scripts/regenerate_expected.sh
```
