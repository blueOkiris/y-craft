// Implement default functions for GameObjects

#include <iostream>
#include <utility>
#include <Sprite.hpp>
#include <SDL2/SDL.h>
#include <Window.hpp>
#include <GameObject.hpp>

bool CollisionShape::collidesWith(const CollisionShape &other) const {
    switch (shapeType) {
        case CollShapeType::Circle:
            switch (other.shapeType) {
                case CollShapeType::Circle: {
                        const auto a = other.center.first - center.first;
                        const auto b = other.center.second - center.second;
                        const auto c = other.radius + radius;
                        return (a * a) + (b * b) <= (c * c);
                    } break;
                case CollShapeType::Rect: {
                        auto test = std::make_pair(center.first, center.second);
                        SDL_Rect rect = {
                            .x = other.center.first - other.width / 2,
                            .y = other.center.second - other.height / 2,
                            .w = other.width,
                            .h = other.height
                        };
                        if (center.first < rect.x) {
                            // Left of rectangle
                            test.first = rect.x;
                        } else if (center.first > rect.x) {
                            // Right of rectangle
                            test.first = rect.x + rect.w;
                        }
                        if (center.second < rect.y) {
                            // Above rect
                            test.second = rect.y;
                        } else if (center.second > rect.y) {
                            // Below
                            test.second = rect.y + rect.h;
                        }
                        const auto distLat = std::make_pair(
                            center.first - test.first,
                            center.second - test.second
                        );
                        const auto distSqrd =
                            (distLat.first * distLat.first) + (distLat.second * distLat.second);
                        return distSqrd <= radius * radius;
                    } break;
            }
            break;
        case CollShapeType::Rect:
            switch (other.shapeType) {
                case CollShapeType::Circle: {
                        auto test = std::make_pair(other.center.first, other.center.second);
                        SDL_Rect rect = {
                            .x = center.first - width / 2,
                            .y = center.second - height / 2,
                            .w = width,
                            .h = height
                        };
                        if (other.center.first < rect.x) {
                            // Left of rectangle
                            test.first = rect.x;
                        } else if (other.center.first > rect.x) {
                            // Right of rectangle
                            test.first = rect.x + rect.w;
                        }
                        if (other.center.second < rect.y) {
                            // Above rect
                            test.second = rect.y;
                        } else if (other.center.second > rect.y) {
                            // Below
                            test.second = rect.y + rect.h;
                        }
                        const auto distLat = std::make_pair(
                            other.center.first - test.first,
                            other.center.second - test.second
                        );
                        const auto distSqrd =
                            (distLat.first * distLat.first) + (distLat.second * distLat.second);
                        return distSqrd <= other.radius * other.radius;
                    } break;
                    break;
                case CollShapeType::Rect: {
                        SDL_Rect r1 = {
                            .x = center.first - width / 2,
                            .y = center.second - height / 2,
                            .w = width,
                            .h = height
                        };
                        SDL_Rect r2 = {
                            .x = other.center.first - other.width / 2,
                            .y = other.center.second - other.height / 2,
                            .w = other.width,
                            .h = other.height
                        };
                        return r1.x + r1.w >= r2.x
                            && r1.x <= r2.x + r2.w
                            && r1.y + r1.h >= r2.y
                            && r1.y <= r2.y + r2.h;
                    } break;
            }
            break;
    }
    return false;
}

GameObject::GameObject(
        const std::string &name,
        const std::pair<double, double> &defPos, const Sprite &defSpr, const CollisionShape &collShape):
            id(name), pos(defPos), spr(defSpr), collider(collShape) {}

void GameObject::render(SDL_Renderer *rndrr, const double elapsedTime) {
    spr.update(elapsedTime);
    spr.render(rndrr, std::make_pair(static_cast<int>(pos.first), static_cast<int>(pos.second)));
}

void drawCircle(SDL_Renderer *rndrr, std::pair<int, int> center, int radius) {
    const int diameter = (radius * 2);

    int x = (radius - 1);
    int y = 0;
    int tx = 1;
    int ty = 1;
    int error = (tx - diameter);

    while (x >= y) {
        // Each of the following renders an octant of the circle
        SDL_RenderDrawPoint(rndrr, center.first + x, center.second - y);
        SDL_RenderDrawPoint(rndrr, center.first + x, center.second + y);
        SDL_RenderDrawPoint(rndrr, center.first - x, center.second - y);
        SDL_RenderDrawPoint(rndrr, center.first - x, center.second + y);
        SDL_RenderDrawPoint(rndrr, center.first + y, center.second - x);
        SDL_RenderDrawPoint(rndrr, center.first + y, center.second + x);
        SDL_RenderDrawPoint(rndrr, center.first - y, center.second - x);
        SDL_RenderDrawPoint(rndrr, center.first - y, center.second + x);

        if (error <= 0) {
            y++;
            error += ty;
            ty += 2;
        }
        if (error > 0) {
            x--;
            tx += 2;
            error += (tx - diameter);
        }
    }
}

void GameObject::debugRenderCollider(SDL_Renderer *rndrr) const {
    switch (collider.shapeType) {
        case CollShapeType::Circle:
            drawCircle(
                rndrr,
                std::make_pair(
                    collider.center.first + pos.first,
                    collider.center.second + pos.second
                ), collider.radius
            );
            break;
        case CollShapeType::Rect: {
                SDL_Rect box = {
                    .x = collider.center.first - collider.width + static_cast<int>(pos.first),
                    .y = collider.center.second - collider.height + static_cast<int>(pos.second),
                    .w = collider.width,
                    .h = collider.height
                };
                SDL_SetRenderDrawColor(rndrr, 255, 0, 255, 50);
                SDL_RenderDrawRect(rndrr, &box);
                SDL_SetRenderDrawColor(
                    g_window.rndrr(),
                    g_window.bgColor.r, g_window.bgColor.g, g_window.bgColor.b, g_window.bgColor.a
                );
            } break;
        default:
            break;
    }
}

