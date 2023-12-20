// Define a generic class that's auto-used that can be overwritten

#pragma once

#include <utility>
#include <memory>
#include <string>
#include <map>
#include <SDL2/SDL.h>
#include <engine/Sprite.hpp>

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
            const std::string &defSpr,
            const std::map<std::string, Sprite> &objSprs,
            const CollisionShape &collShape
        );
        void render(SDL_Renderer *rndrr, const double elapsedTime);
        void debugRenderCollider(SDL_Renderer *rndrr) const;

        // User defined behavior
        virtual std::string tag(void) const = 0;
        virtual void update(
            const double delta, const std::vector<std::shared_ptr<GameObject>> &others
        ) = 0;
        virtual void handleSdlEvent(const SDL_Event &ev) = 0;
        virtual void onCollision(const std::shared_ptr<GameObject> &other) = 0;
        virtual void reset(void) = 0;

        const std::string id;

        std::pair<double, double> pos;
        CollisionShape collider;

    protected:
        std::string _curSpr;
        std::map<std::string, Sprite> _sprs;
};

