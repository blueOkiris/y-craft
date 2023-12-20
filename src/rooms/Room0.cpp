// Implement building room 0 for the example project

#include <engine/Room.hpp>
#include <gameobjs/Player.hpp>
#include <gameobjs/Brick.hpp>
#include <rooms/Room0.hpp>

Room rooms::room0(void) {
    return Room(
        false, std::vector<std::shared_ptr<GameObject>>({
            std::dynamic_pointer_cast<GameObject>(std::make_shared<Player>(
                "player", std::make_pair(256.0, 900.0)
            )), std::dynamic_pointer_cast<GameObject>(std::make_shared<Brick>(
                "brick0", std::make_pair(96.0, 900.0)
            )), std::dynamic_pointer_cast<GameObject>(std::make_shared<Brick>(
                "brick1", std::make_pair(160.0, 1000.0)
            )), std::dynamic_pointer_cast<GameObject>(std::make_shared<Brick>(
                "brick2", std::make_pair(224.0, 1000.0)
            )), std::dynamic_pointer_cast<GameObject>(std::make_shared<Brick>(
                "brick3", std::make_pair(288.0, 1000.0)
            )), std::dynamic_pointer_cast<GameObject>(std::make_shared<Brick>(
                "brick4", std::make_pair(352.0, 1000.0)
            )), std::dynamic_pointer_cast<GameObject>(std::make_shared<Brick>(
                "brick5", std::make_pair(416.0, 1000.0)
            ))
        })
    );
}

