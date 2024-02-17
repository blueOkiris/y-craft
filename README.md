# Y-Craft

## Description

A Rust port of the [x-craft](https://github.com/blueOkiris/x-craft/) engine.

Craft your dream 2D gaming experiences with the X-Craft engine.

Semi-inspired by the systems of the GameMaker engine but fully source-code based (C++), the X-Craft engine hopes to be the ideal engine for indie game devs coming from a programming background.

No longer do you need to figure out how to integrate your abstractions in your engine of choice; build them in regular code and integrate it with the engine by default!

## Build

Requirements:

- Rust
- Cmake
- Probably Linux system
  + May work on other \*nix or even Windows/Mac if set up correctly
  + Only Linux systems are officially supported tho

Note: Nix users can create a valid shell via nix-shell

To make a game:

1. Fork project. You're probably not going to touch the things in the (src|include)/engine/ folders, but you will mess with everything else
2. Add your images and audio to the proper folders
3. Load your images and audio into the proper global hashmaps a la examples in main.cpp
4. Create your custom GameObjects in the (src|include)/gameobjs/ folders to define behavior for your game (see Player.h/cpp and Brick.h/cpp for examples)
   + This is also where you define sprites for animation frames
5. Create rooms to hold instances of GameObjects in the (src|include)/rooms/ folders (see Room0.h/cpp for examples)
6. Replace x-craft with the name of your game in the Makefile and in src/engine/globals.cpp
7. Run make to build your binary

Run `cargo build --release` to build the binary

