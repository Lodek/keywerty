{ lib
, stdenv 
, rustPlatform
, pkg-config
, libevdev
} :

rustPlatform.buildRustPackage {
  name = "vkwrty";
  pname = "vkwrty";

  cargoLock = {
    lockFile = ./Cargo.lock;
  };

  src = ./.;
  nativeBuildInputs = [ pkg-config libevdev stdenv ];
  buildInputs = [ libevdev ];
}
