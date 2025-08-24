use crate::vector3f::Vector3f;
use crate::util;

pub struct Camera { 
    // By default at rotation (0,0,0)
    pub position: Vector3f,
    pub focal_length: f64,
    pub vertical_fov: f64,
    pub aspect_ratio: f64,
}

impl Camera {
    pub fn perspective_projection(&self, point: &Vector3f) -> Vector3f {
        let camera_transform: Vector3f = point.subtract(&self.position);

        let camera_coordinate_point = Vector3f {
            x: (self.focal_length * camera_transform.x) / camera_transform.z,
            y: (self.focal_length * camera_transform.y) / camera_transform.z,
            z: camera_transform.z, // Remember z position
        };

        camera_coordinate_point
    }

    pub fn get_camera_space_dimensions(&self) -> (f64, f64) {
        let half_fov_radians: f64 = util::degrees_to_radians(self.vertical_fov / 2.0);
        let height: f64 = 2.0 * self.focal_length * half_fov_radians.tan();
        let width: f64 = height * self.aspect_ratio;

        (width, height)
    }
}