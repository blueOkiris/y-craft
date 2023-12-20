# X-Craft

## Description

Craft your dream gaming experiences with the X-Craft engine.

Semi-inspired by the systems of the GameMaker engine but fully source-code based (C++), the X-Craft engine hopes to be the ideal engine for indie game devs coming from a programming background.

No longer do you need to figure out how to integrate your abstractions in your engine of choice; build them in regular code and integrate it with the engine by default!

## Building

Requirements:

- SDL, SDL_image, and SDL_mixer
- gcc
- make

Nix users can create a valid shell via `nix-shell`

1. Add your sprites and audio to the proper folders
2. Add custom Game Objects to the `src/` and `include/` directories (use `Player.h/cpp` and `Brick.h/cpp` as reference)
3. Open up the `main.cpp` file and create rooms to store your game objects (pre-loaded with an example)
4. Rename the engine in the Makefile and adjust any constants as necessary
5. Run `make` to build

