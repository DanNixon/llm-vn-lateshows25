{
  pkgs,
  inputs,
  ...
}: let
  pkgs-unstable = import inputs.nixpkgs-unstable {system = pkgs.stdenv.system;};
in {
  packages = with pkgs; [
    # Code formatting tools
    treefmt
    alejandra
    mdl
    rustfmt

    # Rust toolchain
    rustup
    cargo-cross

    pkg-config
    systemd

    # Embedded tools
    pkgs-unstable.probe-rs
    picotool

    # LLM stuff
    pkgs-unstable.ollama
  ];
}
