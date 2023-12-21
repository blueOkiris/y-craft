// Entry point for play-builder engine

#include <engine/globals.hpp>
#include <rooms/title.hpp>

int main(void) {
    // Load audio

    // Load images
    globals::images.insert({
        "title.png", std::make_shared<Image>("img/title.png", globals::win.rndrr())
    });

    // Load fonts
    globals::fonts.insert({
        { "geist", std::make_shared<Font>("fonts/Geist/GeistVariableVF.ttf", 20) }
    });

    // Define rooms w/ Game Objects
    globals::rooms.insert({ "title", rooms::titleScreen() });

    // Start the game!
    globals::win.run("title");
    return 0;
}
