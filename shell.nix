{pkgs ? import <nixpkgs> {}}:
with pkgs;
  mkShell {
    nativeBuildInputs = [rustc cargo];
    buildInputs = [rust-analyzer rustfmt];
  }
