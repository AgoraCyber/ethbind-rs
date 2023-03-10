# ethbind-rs

[!["Crates.io version"](https://img.shields.io/crates/v/ethbind.svg?style=flat-square)](https://crates.io/crates/ethbind) [!["docs.rs docs"](https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square)](https://docs.rs/ethbind)

ethbind-rs is an ethereum contract binding code generation framework for arbitrary programming languages.

## Generator

The binding processor generates arbitrary programming languages codes by calling the corresponding `generator`,The code generator is a rust structure that implements [`Generator`](gen/src/lib.rs) trait.

So far, the only official generator is the `rust` bind code [`Generator`](rust/src/gen/mod.rs), you can easily use this generator in your Rust code in two ways:

### via proc-macro

Using the builtin [`proc-macro`](https://doc.rust-lang.org/reference/procedural-macros.html) contract!($binder_json_path,$abi_json_path) to directly derive contract bind interface in your rust code, e.g:

```rust
contract!(include_str!("xxx/binder.json"),include_str!("xxx/Lock.json"));
```

The above line of rust code will generating `Lock` contract bind codes **in place** via loading contract abi from `Lock.json` file.

### via build.rs

Of course, you can directly call binding processor in build.rs:

#### First, Add ethbind-rs in Cargo.toml

```toml
[build-dependencies]
ethbind-rs = "^0.1"
```

#### Second, Add blow lines of code in build.rs

```rust

/// + use rust `gen_codes` fn
use ethbind::gen::{ JsonRuntimeBinder,SaveTo };
use ethbind::rust::*;

fn main() {
    

    println!("cargo:rerun-if-changed=sol");

    // + load runtime type binding information
    let runtime_binder: JsonRuntimeBinder = include_str!("xxx/binder.json").parse().expect("Load binder information");

    // + define output directory
    let output_dir = "src/sol";

    let mut contracts = BindingBuilder::new((RustGenerator::default(), runtime_binder))
            .bind_hardhat(include_str!("xxx/Lock.json"))
            .bind_hardhat(include_str!("xxx/Swap.json"))
            .finalize()
            .expect("Generate data");
    
    contracts.pretty().expect("Pretty");

    contracts.save_to(output_dir).expect("Save generated");

    // other codes..
}
```
