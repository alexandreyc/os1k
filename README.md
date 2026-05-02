# os1k

A Rust implementation of the [Operating System in 1000 Lines][os1k].

## Development Environment

We assume you're working on a UNIX-like operating system.

To develop, build and run you need to install [rustup][rustup] and [QEMU][qemu].

Next, you need to install some additional components:
```shell
rustup target add riscv32i-unknown-none-elf
rustup component add llvm-tools
cargo install cargo-binutils
```

## Development Loop

You can use all the usual Cargo commands to develop.

In particular, you can use `cargo run` to build and run the OS with QEMU.

Also, you can use the various [binutils][binutils] tools to debug and inspect the generated machine code:
```shell
cargo objdump -- -D
cargo nm
cargo readobj -- -S
```

[os1k]: https://operating-system-in-1000-lines.vercel.app/en/
[rustup]: https://rustup.rs/
[qemu]: https://www.qemu.org/
[binutils]: https://en.wikipedia.org/wiki/GNU_Binutils
