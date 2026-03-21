{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    pkg-config
    systemd.dev   # provides libudev
    udev          # runtime udev
    cargo
    rustc
    nodejs
    python3
    python3Packages.pyside6
  ];
}
