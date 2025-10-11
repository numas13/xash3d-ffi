# Raw FFI bindings to Xash3D FWGS engine

`xash3d-ffi` provides all of the definitions necessary to interoperate with
[Xash3D FWGS engine](https://github.com/FWGS/xash3d-fwgs). Bindings are generated statically
to minimize build dependencies.

# Features

* `std` - links to the standard library.
* `libm` - add additional methods to vectors in no-std environments.
* `glam` - use vector types from this crate.

### Features to enable bindings for DLLs

* `all` (*enabled by default*) - bindings to all supported DLLs.
* `client` - bindings to a client DLL.
* `server` - bindings to a server DLL.
* `menu` - bindings to a menu DLL.
* `render` - bindings to a render DLL.

# Usage

Add the following to your `Cargo.toml`:

```toml
[dependencies]
xash3d-ffi = "0.1"
```

# Rust version support

The minimum supported Rust toolchain version is currently Rust 1.64.

# Generate bindings

[bindgen-cli](https://github.com/rust-lang/rust-bindgen) is **required**.

To manually generate bindings run `generate.sh [target]`. The default target is
`i686-unknown-linux-gnu`.

```ignore
cd xash3d-ffi
./generate.sh
```
