{
  pkgs,
  self,
  ...
}:
pkgs.mkShell {
  name = "bibfetch";
  buildInputs = with pkgs; [
    pkg-config
    luajit
  ];
  packages = with pkgs; [
    nixd
    alejandra
    statix
    deadnix
    cargo
    rustToolchains.nightly
    bacon
    luajit
    pkg-config
    lua-language-server
    wit-bindgen
    # (self.packages.${system}.bibfetch.withHandlers (h: [h.doi]))
  ];
}
