{
  self,
  pkgs,
  lib,
  inputs,
  ...
}: rec {
  default = bibfetch;
  bibfetch = pkgs.callPackage ./bibfetch.nix {inherit pkgs inputs lib self;};
}
