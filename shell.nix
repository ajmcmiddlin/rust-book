{nixpkgs ? import ./.nixpkgs.nix}:

let
  mozilla = import (builtins.fetchGit {
    # Descriptive name to make the store path easier to identify
    name = "nixpkgs-mozilla_at_2020-10-29";
    url = https://github.com/mozilla/nixpkgs-mozilla;
    # `git ls-remote https://github.com/mozilla/nixpkgs-mozilla master`
    rev = "8c007b60731c07dd7a052cce508de3bb1ae849b4";
  });

  nixpkgs' = import nixpkgs { overlays = [mozilla]; };

  ruststable = (nixpkgs'.latest.rustChannels.stable.rust.override {
    extensions = [ "rust-src" "rust-analysis" ];
  });
in
  with nixpkgs';
  mkShell {
    buildInputs = [ruststable];
  }