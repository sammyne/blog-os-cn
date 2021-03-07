# blog-os-cn

![minimum rustc](https://img.shields.io/badge/rustc-1.52--nightly%2B-blue)

The blogs related to each topic project are hosted under base url https://sammyne.github.io/_post/blog-os with the same name, like https://sammyne.github.io/_post/blog-os/01-freestanding-rust-binary for topic project 01-freestanding-rust-binary.

## Quickstart

It's recommended to play within a docker container as follows

```bash
bash play.sh
```

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

- the `rust-toolchain` file under each topic project (such as [01-freestanding-rust-binary](./01-freestanding-rust-binary/rust-toolchain)) specify the exact version rust to use, so as to make everything reproducible.
