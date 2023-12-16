# Unofficial TEOS-10 GSW Oceanographic Toolbox in Rust

GSW for microcontrollers and full computers.

Note that we do follow TEOS-10 manual and references, but this library is not
endorsed by TEOS-10 committee.

The initial motivation for developing GSW-rs was to support pure Rust firmware
for microcontrollers used in autonomous oceanography platforms -- such as
underwater gliders and Argo floats. Rust stands out as an ideal solution due to
its robustness and reliability complemented by an integrated testing system and
a robust package manager to handle dependencies. Our Rust implementation
achieves speeds comparable with C while capitalizing on Rust's explicit and
intuitive syntax which makes it an optimal choice for scientific applications.

We have already implemented approximately 53 of the public functions described
on TEOS-10, and we do intend to eventually cover all of them. While we're
working on that, we'll keep an [inventory list](inventory.md).

## Talks

We presented about goals and progress (as of Apr 2022) at the
[SEA Improving Scientific Software 2022](https://sea.ucar.edu/conference/2022),
[slides available here](https://github.com/castelao/GSW-rs/tree/main/doc/talks).

## Minimum supported Rust version

Currently the minimum supported Rust version is 1.64.0

## Installing

This crate was developed as a library, thus it is intended to be 'imported' by
other libraries or used by applications. The recommended way to install Rust is
using [rustup](https://www.rust-lang.org/tools/install). Check the book
[Rust Programming Language](https://doc.rust-lang.org/book) for more details.
To include GSW in another crate:
```shell
cargo add gsw
```

To be sure it works properly in your platform, clone this repository and run:
`cargo test`
which will compile the library and run all the tests.

### Features

From the Cargo Book: "Cargo 'features' provide a mechanism to express
conditional compilation and optional dependencies.". The features defined in
GSW-rs are:

- **capi**: Include the C-API so that GSW-rs can be accessed as it was a
            C-library. For instance, the other GSW implementations based on
            GSW-C could be linked with GSW-rs instead by using this feature.
- **compat**: Reproduces the GSW-Matlab implementation for compatibility.
- **invalidasnan**: Returns NaN values on failure. The default behavior is to
                    return an error.
- **nodgdz**: Ignores vertical variations of gravity, i.e. no dependency on z.
              This might be useful on some numerical models.
- **std**: Activate the Rust standard library. The default implementation does
           not rely on std so it can run in embedded systems.

For example, to compile it compatible with the official Matlab library:
```shell
cargo build --features compat
```

## Repository structure

For anyone learning Rust, this repository might be overwhelming. We are doing
many things here, hence more files and directories that would be strictly
necessary.

- The `Cargo.toml` and `src` are the fundamental Rust components. The first
  one contains some metadata describing the crate, while the second one groups
  the source code. This is the core of the crate.
  - Note that in `src` we group the modules following the TEOS-10 library
    card. We also isolate the constants, while avoiding repetition.
  - The `src/lib.rs` is the starting point of the library.
  - We split the tests so that those are close to the target functions. Usually
    all the unit tests would go together somewhere, but here, each function has
    so many tests that it would be an extra work to debug and maintain. Also,
    we add reference cases described in the scientific literature.
- Validation tests are grouped outside the `src`, in the `tests`, which uses
  the reference `data`.
- `convert_refdata` is an auxiliary crate. We don't intend to publish that
  one since it is only used to support GSW-rs. The GSW-Matlab provides a
  reference dataset, which we can use to validate our library. Since our goal
  is to also work with microcontrollers, we have to format that dataset in
  such a way that it can be used in microcontrollers, thus validated. There
  you'll find its own `Cargo.toml` and `src`. We use postcard to encode it.
- We use FFI to expose our library, so it can be used outside as if it was a
  C-library. For instance, to test it, we link the official GSW-Python with our
  Rust library (GSW-rs) instead of the traditional GSW-C, and run the Python
  tests. Take a look on `.github/workflows/gsw-python.yml` to see how we link
  GSW-Python with GSW-rs. The directories `utils`, `include`, `assets`, and
  `examples/usage-from-c` are all related to that.
- Integration tests are developed in `tests`, using the specially encoded
  verification values (using `convert_refdata`) in `tests/data` to validate
  our results. Note that there is one file per module, which are small enough
  to fit in the stack memory of resource-limited systems.

## License

Licensed under the 3-Clause BSD License ([LICENSE](LICENSE))

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the [LICENSE](LICENSE), shall be
licensed as above, without any additional terms or conditions.

Contributions should be done through GitHub, by forking the repository,
creating a new branch, and pushing that new branch back as a Pull Request.
Tests covering the new feature or bugfix must be included, and if relevant,
the documentation updated. If not familiar with the procedure, we encourage to
contact us and we will walk you through the process. Every contribution is
valuable and will be recognized.

A note on tests. We don't follow the typical Rust pattern of grouping all the
tests of the module together. As a scientific library, and a large one, we tend
to keep tests right after the target tested, so it is easy to verify if a
certain function covers the desired behavior. Whenever possible, we also add
tests confirming specific values described in the literature.

A note on references: Please review and add the relevant literature for each
function. It is particularly important to verify the coefficients and the
valid range in the original literature.

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
  license = {BSD-3-Clause},
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

- IOC, SCOR and IAPSO, 2010: The international thermodynamic equation of
  seawater - 2010: Calculation and use of thermodynamic properties.
  Intergovernmental - Oceanographic Commission, Manuals and Guides No. 56,
  UNESCO (English), 196 pp.

- McDougall, T.J. and P.M. Barker, 2011: Getting started with TEOS-10 and the
  Gibbs Seawater (GSW) Oceanographic Toolbox, 28pp., SCOR/IAPSO WG127,
  ISBN 978-0-646-55621-5.

As we review and expand this library, we add the relevant specific references
within each function. You can find those in our source code or the manual.
