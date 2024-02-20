# Y-Craft

NOTE: NOT READY FOR PRODUCTION. WAIT FOR v1.0.0!

## Description

A Rust port of the [x-craft](https://github.com/blueOkiris/x-craft/) engine. This is where the ongoing development of the engine will be. X-Craft is a dead project.

Craft your dream 2D gaming experiences with the Y-Craft engine.

Semi-inspired by the systems of the GameMaker engine but fully source-code based (Rust), the Y-Craft engine hopes to be the ideal engine for indie game devs coming from a programming background.

No longer do you need to figure out how to integrate your abstractions into your engine of choice; build them in regular code and integrate it with the engine by default!

## How it Works

There are few different pieces to a Y-Craft game.

First, you load your resources: images, fonts, and audio files. You simply provide the source and a few parameters into the run function.

Then there are game objects. Game Objects are custom structs that you define and provide behavior for. There are ways for them to interact with each other through collisions and an update function, but generally they are independent. They can be created with sprites containing frames of animations. See the examples for a look at object design.

Then there are rooms. Rooms are simply collections of objects. Rooms can be set to have constant state (persistent) or have them reset on transition.

Resources, Rooms, and Game Objects are all indexed by your own enums. Containers of rooms and resources are passed into the `app::run` function to start the loop.

## Build

Requirements:

- [Rust](https://rustup.rs/)
- SDL2 dev libraries
- Linux\*
   + \*It should work on \*nix, MacOS, and Windows (tested on this too), but only Linux is officially supported. Other platforms may not get help in issues

Notes:

- Nix users can create a valid shell via nix-shell. Easy peasy.
- For Windows users it's never so easy-peasy. To get the SDL2 dev libraries they need to:
   + Download the "\*-devel-\*-VC.zip" files from the respective release pages for the [SDL2](https://github.com/libsdl-org/SDL/releases), [SDL2_mixer](https://github.com/libsdl-org/SDL_mixer/releases), [SDL2_image](https://github.com/libsdl-org/SDL_mixer/releases), and [SDL2_ttf](https://github.com/libsdl-org/SDL_ttf/releases) libraries
   + Extract the ".lib" and ".dll" files, usually under "lib/x64/"
   + Place them in your Rust library folder. If using rustup as linked above, put them in `C:\Users\<your user>\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\x86_64-pc-windows-msvc`
   + For more info, consult the README of the [sdl2 crate GitHub](https://github.com/Rust-SDL2/rust-sdl2)

To just build the library, do `cargo build --release`.

To run the examples, do `cargo run --release --example <example-name>`. The binary for the examples will be in `./target/release/<your target>/examples/<example-name>` if you wish to run it that way.

To include in your Rust project, I recommend just `y-craft = "0.<version>"` and ignore the latest release as no releases will have breaking changes, only true version bumps.

If you want to distribute your application, you will need to make sure the target system receives SDL2, SDL2_mixer, SDL2_image, and SDL2_ttf. On Linux, this is simple as you can make a package and put the dependencies the distro's SDL libraries. On Windows, you will need to place the various .dlls in the same folder as the binary.

