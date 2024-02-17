// Set up the title screen room

#include <vector>
#include <memory>
#include <engine/GameObject.hpp>
#include <engine/Room.hpp>
#include <gameobjs/TitleScreenImage.hpp>
#include <rooms/title.hpp>

Room rooms::titleScreen(void) {
    return Room(
        false, std::vector<std::shared_ptr<GameObject>>({
            std::dynamic_pointer_cast<GameObject>(std::make_shared<TitleScreenImage>())
        })
    );
}

