{pkgs ? import <nixpkgs> {}}:
with pkgs;
  mkShell {
    nativeBuildInputs = [rustc cargo cmake];
    buildInputs = [rustfmt pkg-config fontconfig];
    LD_LIBRARY_PATH = lib.makeLibraryPath [wayland libxkbcommon freetype fontconfig];
  }
