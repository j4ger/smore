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
        common-libs = with pkgs; [
          curl
          fontconfig
          freetype
          libgit2
          openssl
          sqlite
          zlib
          zstd
          alsa-lib
          libxkbcommon
          wayland
          vulkan-loader
          xorg.libX11
          xorg.libxcb
        ];
        libPath = with pkgs; lib.makeLibraryPath ([
       ] ++ common-libs);
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

        devShell = with pkgs; mkShell.override {
          stdenv = pkgs.stdenvAdapters.useMoldLinker pkgs.clangStdenv;
        }{
          buildInputs = [
          ] ++ common-libs;
          nativeBuildInputs = [
            cargo
            rust-analyzer
            rustPackages.clippy
            rustc
            rustfmt
            tokei
            pkg-config
            cmake
          ];
          RUST_SRC_PATH = rustPlatform.rustLibSrc;
          LD_LIBRARY_PATH = libPath;
        };
      });
}
