{ pkgs ? import <nixpkgs> { } }:

with pkgs; mkShell {
    buildInputs = [
        gcc
        gnumake
        ccls
        gdb
    ];
    nativeBuildInputs = [
        SDL2
        SDL2_image
        SDL2_ttf
        SDL2_mixer
    ];
    shellHook = ''
    '';
}

