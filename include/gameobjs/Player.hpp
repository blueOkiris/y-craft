// Example GameObject type definition

#pragma once

#include <utility>
#include <vector>
#include <string>
#include <memory>
#include <engine/GameObject.hpp>

// Example GameObject type definition
class Player: public GameObject {
    public:
        Player(
            const std::string &name,
            const std::pair<double, double> &defPos
        );
        std::string tag(void) const override;
        void update(
            const double delta, const std::vector<std::shared_ptr<GameObject>> &others
        ) override;
        void handleSdlEvent(const SDL_Event &ev) override;
        void onCollision(const std::shared_ptr<GameObject> &other) override;
        void reset(void) override;

        const double moveSpd = 512.0;
        const double jumpSpd = 1500.0;
        const double acc = 50.0;
        const double gravity = 4096.0;
        const double extraGrav = 13000.0;

    private:
        std::pair<double, double> _vel;
        bool _right;
        bool _left;
        std::string _sprName;
        bool _grounded;
        bool _extraGrav;
        bool _facingRight;
        std::pair<double, double> _startPos;
};

