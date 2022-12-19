/**
 * Bounding box / groups
 */
use crate::tuple::*;

pub struct Bounds {
    pub min: Point,
    pub max: Point,
}

impl Default for Bounds {
    fn default() -> Self {
        Bounds {
            min: point(-1.0, -1.0, -1.0),
            max: point(1.0, 1.0, 1.0),
        }
    }
}
