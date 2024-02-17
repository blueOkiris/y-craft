// Implement common math functions

#include <engine/math.hpp>

double math::lerp(const double val, const double approach, const double weight) {
    return val * (1.0 - weight) + (approach * weight);
}

