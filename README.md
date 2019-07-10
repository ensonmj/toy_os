# toy_os
just a toy, copy from https://os.phil-opp.com/ step by step.

## dev dependencies

- `cargo install cargo-xbuild` for cross-compiles **core** and other built-in libraries.
the command depends on the rust source code, which we can install with `rustup component add rust-src`. Now we can build with **xbuild** `cargo xbuild --target x86_64-toy_os.json` or `cargo xbuild` with default target in **.cargo/config**

- `cargo instll bootimage --version "^0.7.3"` which depends on **llvm-tools-preview**, we can install with `rustup component add llvm-tools-preview`. Now we can create a bootable dist image by executing `cargo bootimage`

- **qumu** for test. `qemu-system-x86_64 -drive format=raw,file=bootimage-toy_os.bin` or `cargo xrun` with config in **.cargo/config**