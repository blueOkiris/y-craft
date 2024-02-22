//! Implement collision shapes and handle collisions between them

use sdl2::rect::Rect;

/// Colliders that attach to GameObjects. Support Circle and Rect colliders
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum CollisionShape {
    Circle {
        center: (i32, i32),
        radius: u32
    }, Rect {
        center: (i32, i32),
        size: (u32, u32)
    }, Polygon {
        center: (i32, i32),
        points: Vec<(i32, i32)>
    }
}

impl CollisionShape {
    pub fn collides_with(&self, other: &CollisionShape) -> bool {
        match self {
            CollisionShape::Circle { center, radius } => {
                match other {
                    CollisionShape::Circle { center: other_center, radius: other_radius } => {
                        let a = other_center.0 - center.0;
                        let b = other_center.1 - center.1;
                        let c = other_radius + radius;
                        ((a * a) + (b * b)) as u32 <= (c * c)
                    }, CollisionShape::Rect { center: other_center, size: other_size } => {
                        let mut test = center.clone();
                        let rect = Rect::new(
                            other_center.0 - other_size.0 as i32 / 2,
                            other_center.1 - other_size.1 as i32 / 2,
                            other_size.0,
                            other_size.1
                        );
                        if center.0 < rect.x {
                            test.0 = rect.x;
                        } else if center.0 > rect.x + rect.w {
                            test.0 = rect.x + rect.w;
                        }
                        if center.1 < rect.y {
                            test.1 = rect.y;
                        } else if center.1 > rect.y + rect.h {
                            test.1 = rect.y + rect.h;
                        }
                        let dist_lat = (center.0 - test.0, center.1 - test.1);
                        let dist_sqrd = (dist_lat.0 * dist_lat.0) + (dist_lat.1 * dist_lat.1);
                        dist_sqrd as u32 <= radius * radius
                    }, CollisionShape::Polygon { center: other_center, points } => {
                        for point in points.iter() {
                            let dists = (
                                other_center.0 + point.0 - center.0,
                                other_center.1 + point.1 - center.1
                            );
                            let dist_sqrd = dists.0 * dists.0 + dists.1 + dists.1;
                            if (dist_sqrd as u32) < radius * radius {
                                return true;
                            }
                        }
                        false
                    }
                }
            }, CollisionShape::Rect { center, size } => {
                match other {
                    CollisionShape::Circle { center: other_center, radius: other_radius } => {
                        let mut test = other_center.clone();
                        let rect = Rect::new(
                            center.0 - size.0 as i32 / 2,
                            center.1 - size.1 as i32 / 2,
                            size.0,
                            size.1
                        );
                        if other_center.0 < rect.x {
                            test.0 = rect.x;
                        } else if other_center.0 > rect.x + rect.w {
                            test.0 = rect.x + rect.w;
                        }
                        if other_center.1 < rect.y {
                            test.1 = rect.y;
                        } else if other_center.1 > rect.y + rect.h {
                            test.1 = rect.y + rect.h;
                        }
                        let dist_lat = (other_center.0 - test.0, other_center.1 - test.1);
                        let dist_sqrd = (dist_lat.0 * dist_lat.0) + (dist_lat.1 * dist_lat.1);
                        dist_sqrd as u32 <= other_radius * other_radius
                    }, CollisionShape::Rect { center: other_center, size: other_size } => {
                        let r1 = Rect::new(
                            center.0 - size.0 as i32 / 2,
                            center.1 - size.1 as i32 / 2,
                            size.0,
                            size.1
                        );
                        let r2 = Rect::new(
                            other_center.0 - other_size.1 as i32 / 2,
                            other_center.1 - other_size.1 as i32 / 2,
                            other_size.0,
                            other_size.1
                        );
                        r1.x + r1.w >= r2.x
                            && r1.x <= r2.x + r2.w
                            && r1.y + r1.h >= r2.y
                            && r1.y <= r2.y + r2.h
                    }, CollisionShape::Polygon { center: other_center, points } => {
                        for point in points.iter() {
                            let transformed_point = (
                                other_center.0 + point.0,
                                other_center.1 + point.1
                            );
                            let (left, right) = (
                                center.0 - size.0 as i32 / 2,
                                center.0 + size.0 as i32 / 2
                            );
                            let (top, bottom) = (
                                center.1 - size.1 as i32 / 2,
                                center.1 + size.1 as i32 / 2
                            );
                            if transformed_point.0 >= left
                                    && transformed_point.0 <= right
                                    && transformed_point.1 >= top
                                    && transformed_point.1 <= bottom {
                                return true;
                            }
                        }
                        false
                    }
                }
            }, CollisionShape::Polygon { center, points } => {
                match other {
                    CollisionShape::Circle { center: other_center, radius } => {
                        for point in points.iter() {
                            let dists = (
                                center.0 + point.0 - other_center.0,
                                center.1 + point.1 - other_center.1
                            );
                            let dist_sqrd = dists.0 * dists.0 + dists.1 + dists.1;
                            if (dist_sqrd as u32) < radius * radius {
                                return true;
                            }
                        }
                        false
                    }, CollisionShape::Rect { center: other_center, size } => {
                        for point in points.iter() {
                            let transformed_point = (
                                center.0 + point.0,
                                center.1 + point.1
                            );
                            let (left, right) = (
                                other_center.0 - size.0 as i32 / 2,
                                other_center.0 + size.0 as i32 / 2
                            );
                            let (top, bottom) = (
                                other_center.1 - size.1 as i32 / 2,
                                other_center.1 + size.1 as i32 / 2
                            );
                            if transformed_point.0 >= left
                                    && transformed_point.0 <= right
                                    && transformed_point.1 >= top
                                    && transformed_point.1 <= bottom {
                                return true;
                            }
                        }
                        false
                    }, CollisionShape::Polygon { center: other_center, points: other_points } => {
                        // Iterate over each edge of the first polygon
                        for i in 0..points.len() {
                            let p1 = points[i];
                            let p2 = points[(i + 1) % points.len()];

                            // Calculate the normal of the edge
                            let normal = ((p2.1 - p1.1), -(p2.0 - p1.0));

                            // Project both polygons onto the normal
                            let (min1, max1) = project_polygon(&points, &normal, &center);
                            let (min2, max2) = project_polygon(
                                &other_points, &normal, &other_center
                            );

                            // Check for overlap on the projected axis
                            if max1 < min2 || max2 < min1 {
                                return false;
                            }
                        }
                        true
                    }
                }
            }
        }
    }
}

fn project_polygon(points: &[(i32, i32)], axis: &(i32, i32), center: &(i32, i32)) -> (i32, i32) {
    let mut min = i32::MAX;
    let mut max = i32::MIN;
    for point in points.iter() {
        let proj_point = (point.0 + center.0, point.1 + center.1);
        let dot_prod = proj_point.0 * axis.0 + proj_point.1 * axis.1;
        if dot_prod < min {
            min = dot_prod;
        }
        if dot_prod > max {
            max = dot_prod;
        }
    }
    (min, max)
}

