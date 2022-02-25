{

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "utils";
      };
    };

    naersk = {
      url = "github:nix-community/naersk";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "utils";
      };
    };
  };

  outputs = { self, nixpkgs, naersk, rust-overlay, utils }:
    utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        rust = pkgs.rust-bin.stable.latest.default.override {
          #extensions = [ "rust-src" ];
          #targets = [ "x86_64-unknown-linux-musl" ];
          targets = [ "wasm32-wasi" "wasm32-unknown-unknown" "wasm32-unknown-emscripten" ];
        };
        naersk-lib = naersk.lib."${system}".override {
          cargo = rust;
          rustc = rust;
        };
      in

      with pkgs;
      {
        defaultPackage = naersk-lib.buildPackage {
          pname = "gsw";
          root = ./.;
        };

        devShell = mkShell {
          nativeBuildInputs = [
            clang_13
          ];

          buildInputs = [
            rust
            openssl
            pkgconfig

            llvmPackages_13.libclang
            llvmPackages_13.libcxxClang

            git
            gdb

            cargo-watch
            cargo-c
            cargo-asm
            cargo-outdated
            cargo-criterion
            ripgrep

            (python39.withPackages (ps: with ps; [ virtualenv setuptools mypy tox ]))
          ];

          BINDGEN_EXTRA_CLANG_ARGS = "-isystem ${llvmPackages_13.libclang.lib}/lib/clang/${lib.getVersion clang}/include";
          LIBCLANG_PATH = "${llvmPackages_13.libclang.lib}/lib";

          # workaround for https://github.com/NixOS/nixpkgs/blob/48dfc9fa97d762bce28cc8372a2dd3805d14c633/doc/languages-frameworks/python.section.md#python-setuppy-bdist_wheel-cannot-create-whl
          SOURCE_DATE_EPOCH = 315532800; # 1980
        };
      });
}
