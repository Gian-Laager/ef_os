# EF OS

This is a school project for learning how to write an operating system in Rust. It is in no way feature complete or stable.

## Features
- Heap allocations (support for String, Vec, Box, etc.)
- Printing with the normal `println!` function to VGA text mode
- Printing with ANSI colors using `"\x1b[..;..m"` where .. are the normal ANSI 8bit colors.
- Test suite with the `os_test` macro (can be run with `cargo test`)
- Panicking with back trace if available
- Interrupts (technically supported but not used with `x86_64::software_interrupt!` and `x86_64::set_general_handler!`)

The kernel demonstrates most of these features in an overcomplicated fizz buzz implementation.


## Building

This project requires a nightly version of Rust because it uses some unstable features. At least nightly _2020-07-15_ is required for building. You might need to run `rustup update nightly --force` to update to the latest nightly even if some components such as `rustfmt` are missing it.

You can build the project by running:

```
cargo build
```

To create a bootable disk image from the compiled kernel, you need to install the [`bootimage`] tool:

[`bootimage`]: https://github.com/rust-osdev/bootimage

```
cargo install bootimage
```

After installing, you can create the bootable disk image by running:

```
cargo bootimage
```

This creates a bootable disk image in the `target/x86_64-blog_os/debug` directory.

Please file an issue if you have any problems.

## Running

You can run the disk image in [QEMU] through:

[QEMU]: https://www.qemu.org/

```
cargo run
```

[QEMU] and the [`bootimage`] tool need to be installed for this.

You can also write the image to an USB stick for booting it on a real machine. On Linux, the command for this is:

```
dd if=target/x86_64-ef_os/debug/bootimage-ef_os.bin of=/dev/sdX && sync
```

Where `sdX` is the device name of your USB stick. **Be careful** to choose the correct device name, because everything on that device is overwritten.

This should work in theory but until now it has never worked.
