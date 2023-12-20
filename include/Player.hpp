// Example GameObject type definition

#pragma once

#include <utility>
#include <vector>
#include <string>
#include <Sprite.hpp>
#include <GameObject.hpp>
#include <Audio.hpp>

// Example GameObject type definition
class Player: public GameObject {
    public:
        Player(
            const std::string &name,
            const std::pair<double, double> &defPos,
            const Sprite &defSpr,
            const CollisionShape &collShape,
            const Sprite &idleSpr,
            const Sprite &walkSpr
        );
        std::string tag(void) const override;
        void update(
            std::map<std::string, Audio *> &sounds, const double delta,
            const std::vector<GameObject *> &others
        ) override;
        void handleSdlEvent(std::map<std::string, Audio *> &sounds, const SDL_Event &ev) override;
        void onCollision(const GameObject *other) override;
        void reset(void) override;

        const double moveSpd = 512.0;
        const double jumpSpd = 1500.0;
        const double acc = 50.0;
        const double gravity = 4096.0;
        const double extraGrav = 13000.0;
        const Sprite idle;
        const Sprite walk;

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

