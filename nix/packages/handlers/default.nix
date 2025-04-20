{pkgs, ...}: let
  mkPlugin = name:
    pkgs.runCommand "bibfetch-${name}" {
      src = ../../../bibfetch/handlers/${name}.lua;
    }
    # sh
    ''
      mkdir $out/
      cp $src $out/
    '';
in
  pkgs.lib.makeScope pkgs.newScope (selfH: {
    doi = mkPlugin "doi";
  })
