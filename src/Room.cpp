// Implementation of Room-related functions

#include <vector>
#include <GameObject.hpp>
#include <SDL2/SDL.h>
#include <Window.hpp>
#include <Room.hpp>

Room::Room(const bool persists, const std::vector<GameObject *> &roomObjs):
    persistant(persists), gameObjs(roomObjs) {}

void Room::handleSdlEvent(std::map<std::string, Audio *> &sounds, const SDL_Event &ev) {
    for (size_t i = 0; i < gameObjs.size(); i++) {
        gameObjs[i]->handleSdlEvent(sounds, ev);
    }
}

void Room::update(std::map<std::string, Audio *> &sounds, const double deltaTime) {
    for (size_t i = 0; i < gameObjs.size(); i++) {
        gameObjs[i]->update(sounds, deltaTime, gameObjs);
    }

    for (size_t i = 0; i < gameObjs.size(); i++) {
        const auto collider = CollisionShape {
            .shapeType = gameObjs[i]->collider.shapeType,
            .center = std::make_pair(
                gameObjs[i]->collider.center.first
                    + static_cast<int>(gameObjs[i]->pos.first),
                gameObjs[i]->collider.center.second
                    + static_cast<int>(gameObjs[i]->pos.second)
            ), .width = gameObjs[i]->collider.width,
            .height = gameObjs[i]->collider.height
        };
        for (size_t j = 0; j < gameObjs.size(); j++) {
            if (gameObjs[i] == gameObjs[j]) {
                continue;
            }
            const auto otherCollider = CollisionShape {
                .shapeType = gameObjs[j]->collider.shapeType,
                .center = std::make_pair(
                    gameObjs[j]->collider.center.first
                        + static_cast<int>(gameObjs[j]->pos.first),
                    gameObjs[j]->collider.center.second
                        + static_cast<int>(gameObjs[j]->pos.second)
                ), .width = gameObjs[j]->collider.width,
                .height = gameObjs[j]->collider.height
            };
            if (collider.collidesWith(otherCollider)) {
                gameObjs[i]->onCollision(gameObjs[j]);
            }
        }
    }
}

void Room::render(SDL_Renderer *rndrr, const double elapsed) {
    SDL_RenderClear(rndrr);
    for (size_t i = 0; i < gameObjs.size(); i++) {
        gameObjs[i]->render(rndrr, elapsed);
        if (g_window.drawCollisionShapes) {
            gameObjs[i]->debugRenderCollider(rndrr);
        }
    }
}

void Room::reset(void) {
    if (persistant) {
        return;
    }
    for (size_t i = 0; i < gameObjs.size(); i++) {
        gameObjs[i]->reset();
    }
}

