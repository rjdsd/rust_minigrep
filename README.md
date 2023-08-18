## Simple Grep Utility Built In Rust

### How to Use

By running simple commands as below. you may pass Env variable `IGNORE_CASE`

```
cargo run You poem.txt
IGNORE_CASE=0 cargo run YOU poem.txt
IGNORE_CASE=1 cargo run YOU poem.txt
IGNORE_CASE=true cargo run YOU poem.txt
IGNORE_CASE=false cargo run you poem.txt
```

### Unit Test
```
cargo test
```