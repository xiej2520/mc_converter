# Rust Flake

## Building

```sh
git clone https://github.com/xiej2520/mc_converter.git --recurse-submodules
```

With Rust toolchain installed:

```sh
cargo build
```

Using nix:

```sh
nix build '.?submodules=1'#
nix run '.?submodules=1'#
```
