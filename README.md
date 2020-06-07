# Rust-DLL-Example
Example injectable dll written in rust. You could probably clean up the code a bit.

```
rustc --crate-type cdylib dllmain.rs -Clink-arg=kernel32.lib
```
