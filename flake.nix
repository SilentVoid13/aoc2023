{
  description = "aoc2023";

  inputs = {
    nixpkgs.url = "nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    nixpkgs,
    flake-utils,
    fenix,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [fenix.overlays.default];
        };

        toolchain = with fenix.packages.${system};
          combine [
            default.rustc
            default.cargo
            default.clippy
            default.rustfmt
          ];

        shellPkgs = with pkgs; [
          toolchain
          z3
          clang
        ];
      in {
        devShell = pkgs.mkShell {
          packages = shellPkgs;
          LIBCLANG_PATH = "${pkgs.llvmPackages_15.libclang.lib}/lib";
          LD_LIBRARY_PATH = "${pkgs.z3.lib}/lib";
        };
      }
    );
}
