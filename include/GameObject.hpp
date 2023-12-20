// Define a generic class that's auto-used that can be overwritten

#pragma once

#include <utility>
#include <string>
#include <SDL2/SDL.h>
#include <Sprite.hpp>
#include <Audio.hpp>

enum class CollShapeType {
    Circle,
    Rect
};

struct CollisionShape {
    bool collidesWith(const CollisionShape &other) const;

    const CollShapeType shapeType;
    std::pair<int, int> center;
    union {
        int radius;
        struct {
            int width;
            int height;
        };
    };
};

class GameObject {
    public:
        GameObject(
            const std::string &name,
            const std::pair<double, double> &defPos,
            const Sprite &defSpr,
            const CollisionShape &collShape
        );
        void render(SDL_Renderer *rndrr, const double elapsedTime);
        void debugRenderCollider(SDL_Renderer *rndrr) const;

        // User defined behavior
        virtual std::string tag(void) const = 0;
        virtual void update(
            std::map<std::string, Audio *> &sounds, const double delta,
            const std::vector<GameObject *> &others
        ) = 0;
        virtual void handleSdlEvent(
            std::map<std::string, Audio *> &sounds, const SDL_Event &ev
        ) = 0;
        virtual void onCollision(const GameObject *other) = 0;
        virtual void reset(void) = 0;

        std::string id;
        std::pair<double, double> pos;
        Sprite spr;
        CollisionShape collider;
};

