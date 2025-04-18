{
  description = "Minecraft server";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs";

    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    # Personal library
    babel = {
      url = "github:dysthesis/babel";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs = inputs @ {
    self,
    babel,
    nixpkgs,
    treefmt-nix,
    rust-overlay,
    ...
  }: let
    inherit (builtins) mapAttrs;
    inherit (babel) mkLib;
    lib = mkLib nixpkgs;

    # Systems to support
    systems = [
      "aarch64-linux"
      "x86_64-linux"
      "x86_64-darwin"
      "aarch64-darwin"
    ];

    overlays = [
      rust-overlay.overlays.default
      (final: prev: {
        rustToolchains = {
          stable = final.rust-bin.stable.latest.default.override {
            extensions = [
              "rust-src"
              "rust-analyzer"
              "llvm-tools"
            ];
          };
          nightly = final.rust-bin.nightly.latest.default.override {
            extensions = [
              "rust-src"
              "rust-analyzer"
              "miri"
            ];
          };
        };
      })
    ];

    forAllSystems = lib.babel.forAllSystems {inherit systems overlays;};

    treefmt = forAllSystems (pkgs: treefmt-nix.lib.evalModule pkgs ./nix/formatters);
  in
    # Budget flake-parts
    mapAttrs (_: val: forAllSystems val) {
      devShells = pkgs: {default = import ./nix/shell pkgs;};
      # for `nix fmt`
      formatter = pkgs: treefmt.${pkgs.system}.config.build.wrapper;
      # for `nix flake check`
      checks = pkgs: {
        formatting = treefmt.${pkgs.system}.config.build.check self;
      };
      packages = pkgs: import ./nix/packages {inherit self pkgs inputs lib;};
    };
}
