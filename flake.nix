{
  inputs = {
    naersk.url = "github:nmattia/naersk/master";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    utils.url = "github:numtide/flake-utils";
    flake-compat = {
      url = github:edolstra/flake-compat;
      flake = false;
    };
  };

  outputs = { self, nixpkgs, utils, naersk, ... }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        naersk-lib = pkgs.callPackage naersk { };
        libPath = with pkgs; lib.makeLibraryPath [
          xorg.libxcb
          libxkbcommon
          wayland
          vulkan-loader
        ];
     in
      {
        defaultPackage = naersk-lib.buildPackage {
          src = ./.;
          doCheck = true;
          pname = "smore";
          nativeBuildInputs = [ pkgs.makeWrapper ];
          buildInputs = with pkgs; [
            xorg.libxcb
          ];
          postInstall = ''
            wrapProgram "$out/bin/smore" --prefix LD_LIBRARY_PATH : "${libPath}"
          '';
        };

        defaultApp = utils.lib.mkApp {
          drv = self.defaultPackage."${system}";
        };

        devShell = with pkgs; mkShell {
          buildInputs = [
            cargo
            rust-analyzer
            rustPackages.clippy
            rustc
            rustfmt
            tokei

            xorg.libxcb
            libxkbcommon
            wayland
          ];
          RUST_SRC_PATH = rustPlatform.rustLibSrc;
          LD_LIBRARY_PATH = libPath;
        };
      });
}
