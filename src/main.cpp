// Entry point for play-builder engine

#include <SDL2/SDL.h>
#include <string>
#include <memory>
#include <utility>
#include <iostream>
#include <engine/Window.hpp>
#include <engine/Sprite.hpp>
#include <engine/GameObject.hpp>
#include <engine/Room.hpp>
#include <engine/Audio.hpp>
#include <engine/globals.hpp>
#include <rooms/Room0.hpp>

int main(void) {
    // Load audio
    globals::sounds.insert({
        "music", std::make_shared<Audio>("audio/awake10_megaWall.mp3", AudioClipType::Music)
    });
    globals::sounds.insert({
        "jump", std::make_shared<Audio>("audio/sfx_movement_jump10.wav", AudioClipType::Chunk)
    });

    // Load images
    globals::images.insert({
        "character.png", std::make_shared<Image>("img/character.png", globals::win.rndrr()),
    });
    globals::images.insert({
        "brick.png", std::make_shared<Image>("img/brick.png", globals::win.rndrr())
    });

    // Load fonts
    globals::fonts.insert({
        { "geist", std::make_shared<Font>("fonts/Geist/GeistVariableVF.ttf", 20) }
    });

    // Define rooms w/ Game Objects
    globals::rooms.insert({ "rm0", rooms::room0() });

    // Start the game!
    globals::win.run("rm0");
    return 0;
}
