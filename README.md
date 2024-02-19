# Y-Craft

## Description

A Rust port of the [x-craft](https://github.com/blueOkiris/x-craft/) engine. This is where the ongoing development of the engine will be. X-Craft is a dead project.

Craft your dream 2D gaming experiences with the Y-Craft engine.

Semi-inspired by the systems of the GameMaker engine but fully source-code based (Rust), the Y-Craft engine hopes to be the ideal engine for indie game devs coming from a programming background.

No longer do you need to figure out how to integrate your abstractions into your engine of choice; build them in regular code and integrate it with the engine by default!

## How it Works

There are few different pieces to a Y-Craft game.

First, you load your resources: images, fonts, and audio files. You simply provide the source and a few parameters into the run functions.

Then there are game objects. Game Objects are custom structs that you define and provide behavior for. There are ways for them to interact with each other through collisions and an update function, but generally they are independent. They can be created with sprites containing frames of animations. See the examples for a look at object design.

Then there are rooms. Rooms are simply collections of objects. Rooms can be set to have constant state (persistent) or have them reset on transition.

Rooms and resource links are passed into the `app::run` function to start the loop.

## Build

Requirements:

- Rust
- Cmake
- Probably Linux system
  + May work on other \*nix or even Windows/Mac if set up correctly
  + Only Linux systems are officially supported tho

Note: Nix users can create a valid shell via nix-shell

Run `cargo build --release` to build the binary

