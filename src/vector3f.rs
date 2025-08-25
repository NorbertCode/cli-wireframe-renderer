use serde::Deserialize;

use crate::util;

#[derive(Deserialize)]
pub struct Vector3f {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3f {
    pub fn negate(&self) -> Vector3f {
        Vector3f {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }

    pub fn add(&self, other: &Vector3f) -> Vector3f {
        Vector3f {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    pub fn subtract(&self, other: &Vector3f) -> Vector3f {
        self.add(&other.negate())
    }

    pub fn rotate_point(&self, origin: &Vector3f, angle_degrees_clockwise: &Vector3f) -> Vector3f {
        let mut centered_point: Vector3f = self.subtract(origin);
        centered_point = centered_point.rotate_point_around_origin(&angle_degrees_clockwise);

        centered_point.add(&origin)
    }

    pub fn rotate_point_around_origin(&self, angle_degrees_clockwise: &Vector3f) -> Vector3f {
        let angle_rad = Vector3f {
            x: util::degrees_to_radians(angle_degrees_clockwise.x),
            y: util::degrees_to_radians(angle_degrees_clockwise.y),
            z: util::degrees_to_radians(angle_degrees_clockwise.z),
        };

        // X axis rotation
        let mut point = Vector3f {
            x: self.x,
            y: (self.y * angle_rad.x.cos()) + (-self.z * angle_rad.x.sin()),
            z: (self.y * angle_rad.x.sin()) + self.z * angle_rad.x.cos(),
        };

        // Y axis rotation
        point = Vector3f {
            x: (point.x * angle_rad.y.cos()) + (point.z * angle_rad.y.sin()),
            y: point.y,
            z: (-point.x * angle_rad.y.sin()) + (point.z * angle_rad.y.cos()),
        };

        // Z axis rotation
        point = Vector3f {
            x: (point.x * angle_rad.z.cos()) + (-point.y * angle_rad.z.sin()),
            y: (point.x * angle_rad.z.sin()) + (point.y * angle_rad.z.cos()),
            z: point.z
        };

        point
    }
}
