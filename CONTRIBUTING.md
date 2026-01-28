# Contributing

## Run against kdl-rs

```shell
cargo build -p kdl-rs-decoder
cargo run -- --decoder target/debug/kdl-rs-decoder
```

## Regenerate expected JSON files

```shell
scripts/regenerate_expected.sh
```
