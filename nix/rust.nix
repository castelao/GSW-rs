# nix/rust.nix
{ sources ? import ./sources.nix }:
let
  pkgs =
    import sources.nixpkgs { overlays = [ (import sources.rust-overlay) ]; };
    #rustVersion = pkgs.rust-bin.stable."1.37.0".rust.override {
    rustVersion = pkgs.rust-bin.stable.latest.default.override {
    #extensions = [ "rust-src" ];
    #targets = [ "x86_64-unknown-linux-musl" ];
    targets = [ "wasm32-wasi" "wasm32-unknown-unknown" "wasm32-unknown-emscripten"];
  };
in
pkgs.makeRustPlatform {
  cargo = rustVersion;
  rustc = rustVersion;
}
