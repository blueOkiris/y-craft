// Simple "wall" game object

#pragma once

#include <utility>
#include <string>
#include <Sprite.hpp>
#include <GameObject.hpp>
#include <Audio.hpp>

// Example GameObject type definition
class Brick: public GameObject {
    public:
        Brick(
            const std::string &name,
            const std::pair<double, double> &defPos,
            const Sprite &defSpr,
            const CollisionShape &collShape
        );
        std::string tag(void) const override;
        void update(
            std::map<std::string, Audio *> &sounds, const double delta,
            const std::vector<GameObject *> &others
        ) override;
        void handleSdlEvent(std::map<std::string, Audio *> &sounds, const SDL_Event &ev) override;
        void onCollision(const GameObject *other) override;
        void reset(void) override;
};

