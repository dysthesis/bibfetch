{
  rustPlatform,
  cargo,
  rustc,
  luajit,
  pkg-config,
  ...
}:
rustPlatform.buildRustPackage rec {
  name = "bibfetch";
  version = "0.1.0";
  buildInputs = [
    luajit
  ];
  nativeBuildInputs = [
    cargo
    rustc
    pkg-config
  ];
  src = ../../.;
  cargoLock.lockFile = "${src}/Cargo.lock";
}
