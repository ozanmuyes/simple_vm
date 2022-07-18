# rust-analyzer extension issue

Using `#![no_std]`

And having such code to make rustc happy (given no std above);
```rust
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
```

VSCode show this error;
```
found duplicate lang item `panic_impl`
the lang item is first defined in crate `std` (which `test` depends on)
first definition in `std` loaded from /Users/ozan/.rustup/toolchains/stable-x86_64-apple-darwin/lib/rustlib/x86_64-apple-darwin/lib/libstd-98c00dfac2ed63a9.rlib
second definition in the local crate (`simple_vm_rust`) rustc(E0152)
```

## Versions

### VSCode

```shell
$ code --version
1.68.1
30d9c6cd9483b2cc586687151bcbcd635f373630
x64
```

### rust-analyzer

rust-analyzer
v0.3.1123

### rustup

```shell
$ rustup -V
rustup 1.25.1 (bb60b1e89 2022-07-12)
info: This is the version for the rustup toolchain manager, not the rustc compiler.
info: The currently active `rustc` version is `rustc 1.62.0 (a8314ef7d 2022-06-27)`
```

## Relative Issues

* https://github.com/rust-lang/rust-analyzer/issues/10659
* https://github.com/rust-lang/rust-analyzer/issues/1626
