{ naersk, pkgs, ... }:
naersk.buildPackage rec {
  pname = "rubric-merge";
  name = pname;
  src = ../.;
  strictDeps = true;

  OPENSSL_STATIC = "1";
  OPENSSL_LIB_DIR = "${pkgs.pkgsStatic.openssl.out}/lib";
  OPENSSL_INCLUDE_DIR = "${pkgs.pkgsStatic.openssl.dev}/include";

  CARGO_BUILD_TARGET = "x86_64-unknown-linux-musl";
  CARGO_BUILD_RUSTFLAGS = [
    "-C"
    "target-feature=+crt-static"

    # -latomic is required to build openssl-sys for armv6l-linux, but
    # it doesn't seem to hurt any other builds.
    # "-C"
    # "link-args=-static -latomic"
  ];
}
