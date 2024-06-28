# Rust libvmaf wrapper

VMAF is a perceptual video quality assessment based on multi-method fusion developed by the Netflix in for of the [`libvmaf`](https://github.com/Netflix/vmaf). 

This repository consists of two crates [`rust-vmaf-sys`](crates/rust-vmaf-sys), which is used for generating bindings for the libvmaf and [`rust-vmaf`](crates/rust-vmaf/) which exposes Rust idiomatic structures to the raw bindings.

## License

Licensed under MIT license ([LICENSE](LICENSE) or http://opensource.org/licenses/MIT)
