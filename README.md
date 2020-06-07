# Rust-DLL-Example
Example injectable dll written in rust.

```
rustc --crate-type cdylib dllmain.rs -Clink-arg=kernel32.lib
```
