# gstd-fluent

[![Build][build_badge]][build_href]
[![License][lic_badge]][lic_href]
[![Docs][docs_badge]][docs_href]

[build_badge]: https://img.shields.io/github/actions/workflow/status/gear-foundation/gstd-fluent/ci.yml?label=Build
[build_href]: https://github.com/gear-foundation/gstd-fluent/actions/workflows/ci.yml

[lic_badge]: https://img.shields.io/badge/License-MIT-success
[lic_href]: LICENSE

[docs_badge]: https://img.shields.io/badge/Docs-online-5023dd?logo=rust
[docs_href]: https://docs.rs/gstd-fluent/

A wrapper over [gstd](https://github.com/gear-tech/gear/tree/master/gstd)
that provides a fluent interface to interact with the Gear Protocol.

To use the default implementation, you should replace `gstd` with `gstd-fluent` in your `Cargo.toml` file:

```diff
- gstd = "x.y.z"
+ gstd-fluent = "x.y.z"
```

Then update your code like this:

```rust
#![no_std]

use gstd_fluent::{
    self as builder,
    gstd::{self, prelude::*, ActorId},
    //     ^^^^ Useful when working with gstd attribute macros
};

#[no_mangle]
extern "C" fn handle() {
    // Equivalent to `msg::send_with_gas`
    builder::send(ActorId::zero(), String::from("input"))
        .with_value(42)
        .with_gas_limit(1_000_000)
        .execute()
        .expect("failed to send msg");
}
```

For more examples, please visit the [documentation][docs_href].

## License

The source code is licensed under the [MIT license](LICENSE).
