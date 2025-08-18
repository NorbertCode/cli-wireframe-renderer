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

    pub fn rotate_point(&self, origin: &SpacePoint, angle_degrees_clockwise: f64) -> SpacePoint {
        let mut centered_point: SpacePoint = self.subtract(origin);
        centered_point = centered_point.rotate_point_around_origin(angle_degrees_clockwise);

        centered_point.add(&origin)
    }

    fn rotate_point_around_origin(&self, angle_degrees_clockwise: f64) -> SpacePoint {
        let angle_radians: f64 = util::degrees_to_radians(angle_degrees_clockwise);
        let angle_sin: f64 = angle_radians.sin();
        let angle_cos: f64 = angle_radians.cos();

        SpacePoint {
            x: (self.x * angle_cos) + (self.z * angle_sin),
            y: self.y,
            z: (-self.x * angle_sin) + (self.z * angle_cos),
        }
    }
}
