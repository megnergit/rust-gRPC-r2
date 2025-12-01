# Rust gRPC implementation
Understand how rust works with gRPC

## Table of Contents


---
## Motivation

I would like to understand how Rust works with gRPC



## Start project 


```sh 
$ cargo new grpc-server
$ cargo new grpc-client
$ tree .
.
├── LICENSE
├── README.md
├── RUST_GRPC
├── grpc-server
│   ├── Cargo.toml
│   └── src
│       └── main.rs
├── grpc-client
│   ├── Cargo.toml
│   └── src
│       └── main.rs
...

$ mkdir proto
$ touch proto/hello.proto

```


## Source code



---
# END
---
cargo new proto-defs --lib

（※ src/generated/ は .gitignore に入れると良い）