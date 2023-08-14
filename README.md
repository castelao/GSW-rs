# Unofficial TEOS-10 GSW Oceanographic Toolbox in Rust

GSW for microcontrollers.

Note that we do follow TEOS-10 manual and references, but his library is not
endorsed by TEOS-10 committee.

## Talks

We presented about goals and progress (as of Apr 2022) at the
[SEA Improving Scientific Software 2022](https://sea.ucar.edu/conference/2022),
[slides available here](https://github.com/castelao/GSW-rs/tree/main/doc/talks).

## Minimum supported Rust version

Currently the minimum supported Rust version is 1.64.0

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

## Citation

If you use this library we kindly ask to cite all the three following references:

[![DOI](https://zenodo.org/badge/DOI/10.5281/zenodo.5348561.svg)](https://doi.org/10.5281/zenodo.5348561)

Castelao, G., & Irber, L. (2021). Unofficial Gibbs Sea Water Oceanographic
Toolbox of TEOS-10 implemented in Rust (Version 0.1.1) [Computer software].
https://doi.org/10.5281/zenodo.5348561

```text
@software{Castelao_Gibbs_Sea_Water_2021,
  title = {{Gibbs Sea Water Oceanographic Toolbox of TEOS-10 implemented in Rust}},
  author = {Castelao, Guilherme and Irber, Luiz},
  year = {2021},
  license = {MIT OR Apache-2.0},
  version = {0.1.1},
  doi = {10.5281/zenodo.5348561},
  url = {https://github.com/castelao/GSW-rs}
  }
```

Why should you also cite the two following references? We do appreciate
you recognizing our efforts developing this library citing the reference
above, but we can't take any credit for the theory behind it. There is a
long list of publications that resulted in all these relations explored in
this library. Although we encourage you to study and cite the ones relevant
to your work, you should at least cite the two following ones. The manual 56
that summarizes all those publications, and the 'Getting started' publication
which presents how to use the Toolbox.

IOC, SCOR and IAPSO, 2010: The international thermodynamic equation of
seawater - 2010: Calculation and use of thermodynamic properties.
Intergovernmental - Oceanographic Commission, Manuals and Guides No. 56,
UNESCO (English), 196 pp.

McDougall, T.J. and P.M. Barker, 2011: Getting started with TEOS-10 and the
Gibbs Seawater (GSW) Oceanographic Toolbox, 28pp., SCOR/IAPSO WG127,
ISBN 978-0-646-55621-5.

As we review and expand this library, we add the relevant specific references
within each function. You can find those in our source code or the manual.
