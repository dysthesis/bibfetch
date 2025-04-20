{
  rustPlatform,
  cargo,
  rustc,
  ...
}:
rustPlatform.buildRustPackage rec {
  name = "bibfetch-request";
  version = "0.1.0";

  flags = [
    "-p"
    name
    "--target"
    "wasm32-unknown-unknown"
  ];
  nativeBuildInputs = [
    cargo
    rustc
  ];
  src = ../../.;
  cargoLock.lockFile = "${src}/Cargo.lock";
}
