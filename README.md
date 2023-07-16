renamectl
====

A tool for renaming episodes, using in my NAS

## How to build/use

```
rustup default nightly
cargo build --target x86_64-unknown-linux-musl
./target/x86_64-unknown-linux-musl/debug/renamectl --dir {your_dir}
```

## See Also

- [Command line apps in Rust](https://rust-cli.github.io/book/index.html)
