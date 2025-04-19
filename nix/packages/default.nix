{
  self,
  pkgs,
  lib,
  inputs,
  ...
}: rec {
  default = bibfetch;
  bibfetch = pkgs.callPackage ./bibfetch.nix {inherit pkgs inputs lib self;};
  handlers = import ./handlers {inherit pkgs inputs lib self;};
}
