{
  inputs = {
    naersk.url = "github:nix-community/naersk/master";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils, naersk }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        naersk-lib = pkgs.callPackage naersk { };
      in
      {
        # TODO: makewrapper that includes wayland etc.
        smore =  naersk-lib.buildPackage {
          pname = "smore";
          root = ./.;
          nativeBuildInputs = with pkgs; [
            mold
            qt6.full
          ];
          NIX_CFLAGS_LINK = " -fuse-ld=mold";
        };
        devShell = with pkgs; mkShell.override {
          stdenv = pkgs.stdenvAdapters.useMoldLinker pkgs.clangStdenv;
        } {
          buildInputs = [
            cargo
            rustc
            rustfmt
            rustPackages.clippy 
            qt6.full
            bacon
          ];
          RUST_SRC_PATH = rustPlatform.rustLibSrc;
          LD_LIBRARY_PATH = "$LD_LIBRARY_PATH:${ with pkgs; lib.makeLibraryPath [
            wayland
            libxkbcommon
            fontconfig
            libGL
            xorg.libxcb 
            xorg.libXfixes
          ]}";
        };
      }
    );
}
