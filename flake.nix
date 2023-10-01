{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };

  outputs = {
    self,
      nixpkgs,
      flake-utils,
      ...
  }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
      in with pkgs; rec {
        devShell = mkShell rec {
          buildInputs = [
            rustc
            rust-analyzer
            cargo
            rustfmt
            cmake
            pkg-config
            libxkbcommon
            libGL
            libpng
            expat
            fontconfig
            freetype
            freetype.dev
            libGL
            xorg.libX11
            xorg.libXcursor
            xorg.libXi
            xorg.libXrandr
            wayland
          ];
          # LD_LIBRARY_PATH = "${lib.makeLibraryPath buildInputs}";
          LD_LIBRARY_PATH = builtins.foldl' (a: b: "${a}:${b}/lib") "${pkgs.vulkan-loader}/lib" buildInputs;
        };
      });
}
