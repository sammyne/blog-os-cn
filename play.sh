#!/bin/bash

docker run -it --rm                                                     \
  -e RUSTUP_UPDATE_ROOT=https://mirrors.ustc.edu.cn/rust-static/rustup  \
  -e RUSTUP_DIST_SERVER=https://mirrors.ustc.edu.cn/rust-static         \
  -v $PWD:/home/blog-os                                                 \
  -w /home/blog-os                                                      \
  rust:1.50.0-alpine3.13 sh

