{ pkgs, bin, ... }:
pkgs.dockerTools.buildImage {
  name = "ghcr.io/anucssa/rubric-merge";
  tag = "latest";
  copyToRoot = [ bin ];
  config = {
    Cmd = [ "${bin}/bin/rubric-merge" ];
  };
}
