{
  self,
  lib,
  pkgs,
  rustPlatform,
  symlinkJoin,
  cargo,
  rustc,
  luajit,
  pkg-config,
  makeWrapper,
  ...
}: let
  unwrapped = rustPlatform.buildRustPackage rec {
    name = "bibfetch";
    version = "0.1.0";
    buildInputs = [
      luajit
    ];
    flags = ["-p" name];
    nativeBuildInputs = [
      cargo
      rustc
      pkg-config
      makeWrapper
    ];
    src = ../../.;
    cargoLock.lockFile = "${src}/Cargo.lock";
  };
in
  unwrapped
  // {
    withHandlers = f: let
      inherit (lib.babel.pkgs) mkWrapper;
      handlers = f self.packages.${pkgs.system}.handlers;
      handlersDrv = symlinkJoin {
        name = "bibfetch-handlers";
        paths = handlers;
      };
    in
      mkWrapper pkgs unwrapped ''
        wrapProgram $out/bin/bibfetch \
          --set BIBFETCH_HANDLERS_DIR ${handlersDrv}
      '';
  }
