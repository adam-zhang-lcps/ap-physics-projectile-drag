{pkgs ? import <nixpkgs> {}}:
with pkgs;
  mkShell {
    nativeBuildInputs = [rustc cargo cmake];
    buildInputs = [rust-analyzer rustfmt pkg-config fontconfig mold];
    LD_LIBRARY_PATH = lib.makeLibraryPath [wayland libxkbcommon freetype];
  }
