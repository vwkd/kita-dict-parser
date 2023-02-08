# README

The grammar and parser for [kita-dict-data](https://github.com/vwkd/kita-dict-data)



## Usage

- to see success and error

```
cargo run
```

- to see only success

```
cargo run 2> /dev/null
```

- to see only error

```
cargo run 1> /dev/null
```



## Architecture

- `nom` parser in Rust
