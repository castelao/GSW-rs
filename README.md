# TEOS-10 GSW Oceanographic Toolbox in Rust

GSW for microcontrollers.

## Minimum supported Rust version

Currently the minimum supported Rust version is 1.46.0

## Features

From the Cargo Book: "Cargo 'features' provide a mechanism to express
conditional compilation and optional dependencies.". The features defined in
GSW-rs are:

- capi: Include the C-API so that GSW-rs can be accessed as it was a
        C-library. For instance, the other GSW implementations based on
        GSW-C could be linked with GSW-rs instead by using this feature.
- compat: Reproduces the GSW-Matlab implementation for compatibility.
- invalidasnan: Returns NaN values on failure. The default behavior is to
                return an error.
- nodgdz: Ignores vertical variations of gravity, i.e. no dependency on z.
          This might be useful on some numerical models.
- std: Activate the Rust standard library. The default implementation does not
       rely on std so it can run in embedded systems.

For example, to compile it compatible with the official Matlab library:
cargo build --features compat

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
