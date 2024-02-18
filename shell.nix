{ pkgs ? import <nixpkgs> { } }:

let
    unstable = import
        (fetchTarball "https://nixos.org/channels/nixos-unstable/nixexprs.tar.xz") {};
    buildLibs = with pkgs; (with xorg; [
        libX11
        libXcursor
        libXi
        libxkbcommon
        libXrandr
        libXext
        SDL2
        SDL2_ttf
        SDL2_image
        SDL2_mixer
    ]);
in with pkgs; with xorg; mkShell {
    buildInputs = [
        cargo
        cmake
        libX11
        libXcursor
        libXi
        libxkbcommon
        libXrandr
        libXext
        pkg-config
        SDL2
        SDL2_ttf
        SDL2_image
        SDL2_mixer
        unstable.rustc
    ];
    shellHook = ''
        export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${lib.makeLibraryPath buildLibs}"
        export RUST_SRC_PATH="${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}"
    '';
}

