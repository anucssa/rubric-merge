{ craneLib, ... }:
craneLib.buildPackage rec {
  pname = "rubric-merge";
  name = pname;
  src = craneLib.cleanCargoSource ../.;
}
