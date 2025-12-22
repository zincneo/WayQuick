{
  description = "WayQuick flake";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    crane.url = "github:ipetkov/crane";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { nixpkgs, flake-utils, crane, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        craneLib = crane.mkLib pkgs;
        src = craneLib.cleanCargoSource ./.;
        guiLibs = with pkgs;[
          # misc. libraries
          openssl
          pkg-config
          # GUI libs
          libxkbcommon
          libGL
          fontconfig
          # wayland libraries
          wayland
          # x11 libraries
          xorg.libXcursor
          xorg.libXrandr
          xorg.libXi
          xorg.libX11
          libxcb
          # vulkan
          vulkan-loader
        ];
        commonArgs = {
          inherit src;
          strictDeps = true;
          buildInputs = guiLibs;
          nativeBuildInputs = [ pkgs.pkg-config ];
        };

        cargoArtifacts = craneLib.buildDepsOnly commonArgs;

        # 1. 编译应用
        gmenuUnwrapped = craneLib.buildPackage (commonArgs // {
          inherit cargoArtifacts;
          pname = "WayQuick";
          version = "0.1.0";
          propagatedBuildInputs = guiLibs;
        });

        # 2. 应用编译后需要使用wrapProgram处理动态链接库的路径问题
        gmenu = pkgs.stdenv.mkDerivation {
          pname = "WayQuick";
          version = "0.1.0";
          src = gmenuUnwrapped;
          buildInputs = [ pkgs.makeWrapper ];
          installPhase = ''
            mkdir -p $out/bin
            cp ${gmenuUnwrapped}/bin/wayquick $out/bin/wayquick
            wrapProgram $out/bin/wayquick \
              --prefix LD_LIBRARY_PATH : ${pkgs.lib.makeLibraryPath guiLibs}
          '';
        };
      in
      {
        packages.default = gmenu;
        devShells.default = pkgs.mkShell rec {
          buildInputs = guiLibs;
          LD_LIBRARY_PATH = "${pkgs.lib.makeLibraryPath buildInputs}";
        };
      });
}
