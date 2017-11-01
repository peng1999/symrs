# symrs
[![Build Status](https://travis-ci.org/peng1999/symrs.svg?branch=master)](https://travis-ci.org/peng1999/symrs) [![Build status](https://ci.appveyor.com/api/projects/status/huaq6qv2eymp48ng/branch/master?svg=true)](https://ci.appveyor.com/project/peng1999/symrs/branch/master) [![codecov](https://codecov.io/gh/peng1999/symrs/branch/master/graph/badge.svg)](https://codecov.io/gh/peng1999/symrs)

Symbolic computation in Rust

NOTE: THIS PROJECT IS STILL A PROOF-OF-CONCEPT, AND NOT READY FOR PRACTICAL USE.

## Usage

To use `symrs`, include it in your `Cargo.toml`:

```toml
[dependencies.symrs]
git = "https://github.com/peng1999/symrs.git"
version = "0.*"
```

Then include it in your code:

```rust
extern crate symrs;
```

There are a few compilation features:

- `parser`: includes a parser library.

## License

symrs is primarily distributed under the terms of both the MIT license and the Apache License (Version 2.0), with portions covered by various BSD-like licenses.

See [LICENSE-APACHE](LICENSE-APACHE), and [LICENSE-MIT](LICENSE-MIT) for details.
