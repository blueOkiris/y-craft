// Display a static image on the title screen that will move to the next room on enter

#pragma once

#include <utility>
#include <vector>
#include <string>
#include <memory>
#include <engine/GameObject.hpp>

class TitleScreenImage: public GameObject {
    public:
        TitleScreenImage(void);
        std::string tag(void) const override;
        void update(
            const double delta, const std::vector<std::shared_ptr<GameObject>> &others
        ) override;
        void handleSdlEvent(const SDL_Event &ev) override;
        void onCollision(const std::shared_ptr<GameObject> &other) override;
        void reset(void) override;
};

