//! Helpful, static functions for game dev

pub fn lerp(val: f64, approach: f64, weight: f64) -> f64 {
    val * (1.0 - weight) + (approach * weight)
}

