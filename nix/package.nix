{ craneLib, pkgs, ... }:
craneLib.buildPackage rec {
  pname = "rubric-merge";
  name = pname;
  src = craneLib.cleanCargoSource ../.;

  nativeBuildInputs = with pkgs; [ pkg-config ];
  buildInputs = with pkgs; [ openssl ];
}
