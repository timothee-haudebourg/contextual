# Contextual

[![CI](https://github.com/timothee-haudebourg/contextual/workflows/CI/badge.svg)](https://github.com/timothee-haudebourg/contextual/actions)
[![Crate informations](https://img.shields.io/crates/v/contextual.svg?style=flat-square)](https://crates.io/crates/contextual)
[![License](https://img.shields.io/crates/l/contextual.svg?style=flat-square)](https://github.com/timothee-haudebourg/contextual#license)
[![Documentation](https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square)](https://docs.rs/contextual)

A small crate to deal with data in context.

```rust
use contextual::{WithContext, AsRefWithContext, DisplayWithContext};
use std::fmt;

/// Index of an element in some array.
pub struct Index(usize);

impl<T, C: AsRef<[T]>> AsRefWithContext<T, C> for Index {
  fn as_ref_with<'a>(&'a self, context: &'a C) -> &'a T {
    &context.as_ref()[self.0]
  }
}

impl<'a, C: AsRef<[&'a str]>> DisplayWithContext<C> for Index {
  fn fmt_with(&self, context: &C, f: &mut fmt::Formatter) -> fmt::Result {
    use fmt::Display;
    context.as_ref()[self.0].fmt(f)
  }
}

let i = Index(1);
let context = ["a", "b", "c"];

print!("index: {}", i.with(&context));
assert_eq!(*i.with(&context).as_ref(), "b")
```

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
