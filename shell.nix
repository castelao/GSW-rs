let
  sources = import ./nix/sources.nix;
  rustPlatform = import ./nix/rust.nix { inherit sources; };
  pkgs = import sources.nixpkgs { overlays = [ (import sources.rust-overlay) ]; };
in
  with pkgs;

  pkgs.mkShell {
    buildInputs = [
      rustPlatform.rust.cargo
      git
      stdenv.cc.cc.lib
      llvmPackages.libclang
      cmake
      rust-bindgen
      cargo-watch
      cargo-c
      cargo-asm
      gdb
      (python39.withPackages(ps: with ps; [ virtualenv setuptools mypy ]))

      # convenience tools
      tree
      bat
      ripgrep
    ];

    shellHook = ''
       # workaround for https://github.com/NixOS/nixpkgs/blob/48dfc9fa97d762bce28cc8372a2dd3805d14c633/doc/languages-frameworks/python.section.md#python-setuppy-bdist_wheel-cannot-create-whl
       export SOURCE_DATE_EPOCH=315532800 # 1980
       export LD_LIBRARY_PATH="${stdenv.cc.cc.lib}/lib64:$LD_LIBRARY_PATH";
       export LIBCLANG_PATH="${llvmPackages_11.libclang.lib}/lib";
    '';
  }
