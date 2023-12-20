// Simple "wall" game object

#pragma once

#include <utility>
#include <string>
#include <memory>
#include <engine/Sprite.hpp>
#include <engine/GameObject.hpp>

// Example GameObject type definition
class Brick: public GameObject {
    public:
        Brick(
            const std::string &name,
            const std::pair<double, double> &defPos
        );
        std::string tag(void) const override;
        void update(
            const double delta,
            const std::vector<std::shared_ptr<GameObject>> &others
        ) override;
        void handleSdlEvent(const SDL_Event &ev) override;
        void onCollision(const std::shared_ptr<GameObject> &other) override;
        void reset(void) override;
};

