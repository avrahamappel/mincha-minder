{
  description = "Mincha Minder: generate a calendar of your mincha events";

  inputs = {
    cargo2nix.url = "github:cargo2nix/cargo2nix/release-0.11.0";
    flake-utils.follows = "cargo2nix/flake-utils";
    nixpkgs.follows = "cargo2nix/nixpkgs";
  };

  outputs = { nixpkgs, cargo2nix, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ cargo2nix.overlays.default ];

        pkgs = import nixpkgs { inherit system overlays; };

        rustPkgs = pkgs.rustBuilder.makePackageSet {
          packageFun = import ./Cargo.nix;
          rustVersion = "1.73.0";
        };
      in
      {
        devShells.default = rustPkgs.workspaceShell {
          packages = [
            pkgs.libiconv
            pkgs.rust-analyzer
          ];
        };

        packages.default = (rustPkgs.workspace.mincha-minder { }).bin;
      });
}
