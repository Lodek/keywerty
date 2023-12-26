let
  pkgs = import <nixpkgs> { };
in
{
  vkwrty = pkgs.callPackage ./vkwrty.nix { };
}
