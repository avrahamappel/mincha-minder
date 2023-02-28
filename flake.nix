{
  description = "Mincha Minder: generate a calendar of your mincha events";

  inputs = {
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, rust-overlay, ... }:
    let
      inherit (rust-overlay.inputs) nixpkgs flake-utils;
    in
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = [
            pkgs.rust-bin.stable.latest.default
          ];
        };
      });
}
