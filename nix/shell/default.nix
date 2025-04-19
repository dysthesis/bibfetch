pkgs:
pkgs.mkShell {
  name = "bibfetch";
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
  ];
}
