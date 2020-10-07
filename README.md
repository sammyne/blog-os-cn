# blog-os-cn

![minimum rustc](https://img.shields.io/badge/rustc-1.49--nightly%2B-blue)

## Topic
- [06. Double Faults](./06-double-faults/README.md): **BROKEN**
- [07. Hardware Interrupts](./07-hardware-interrupts/README.md)

## Heads Up
- the `x86_64-blog_os-9782934370283982507` target may not be installed  
It might be `.cargo/config` missing the following config 

  ```toml
  [unstable]
  build-std = ["core", "compiler_builtins"]
  ```
