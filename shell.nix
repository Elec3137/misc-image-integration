let
  pkgs = import <nixpkgs> { };
in
pkgs.mkShell {
  packages = with pkgs; [
    rustc
    cargo
    clippy
    rustfmt
    rust-analyzer
  ];
}
