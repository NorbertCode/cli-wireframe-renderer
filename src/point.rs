use crate::util;

pub struct SpacePoint {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl SpacePoint {
    pub fn negate(&self) -> SpacePoint {
        SpacePoint {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }

    pub fn add(&self, other: &SpacePoint) -> SpacePoint {
        SpacePoint {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    pub fn subtract(&self, other: &SpacePoint) -> SpacePoint {
        self.add(&other.negate())
    }

    pub fn rotate_point(&self, origin: &SpacePoint, angle_degrees_clockwise: (f64, f64, f64)) -> SpacePoint {
        let mut centered_point: SpacePoint = self.subtract(origin);
        centered_point = centered_point.rotate_point_around_origin(angle_degrees_clockwise);

        centered_point.add(&origin)
    }

    fn rotate_point_around_origin(&self, angle_degrees_clockwise: (f64, f64, f64)) -> SpacePoint {
        let angle_rad: (f64, f64, f64) = (
            util::degrees_to_radians(angle_degrees_clockwise.0),
            util::degrees_to_radians(angle_degrees_clockwise.1),
            util::degrees_to_radians(angle_degrees_clockwise.2),
        );

        // X axis rotation
        let mut point = SpacePoint {
            x: self.x,
            y: (self.y * angle_rad.0.cos()) + (-self.z * angle_rad.0.sin()),
            z: (self.y * angle_rad.0.sin()) + self.z * angle_rad.0.cos(),
        };

        // Y axis rotation
        point = SpacePoint {
            x: (point.x * angle_rad.1.cos()) + (point.z * angle_rad.1.sin()),
            y: point.y,
            z: (-point.x * angle_rad.1.sin()) + (point.z * angle_rad.1.cos()),
        };

        // Z axis rotation
        point = SpacePoint {
            x: (point.x * angle_rad.2.cos()) + (-point.y * angle_rad.2.sin()),
            y: (point.x * angle_rad.2.sin()) + (point.y * angle_rad.2.cos()),
            z: point.z
        };

        point
    }
}
