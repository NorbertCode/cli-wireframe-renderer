use super::point::SpacePoint;
use super::util;

#[derive(PartialEq, Eq)]
pub struct ScreenPoint {
    pub x: i32,
    pub y: i32,
}

pub struct Camera { 
    // By default at rotation (0,0,0)
    pub position: SpacePoint,
    pub focal_length: f64,
    pub vertical_fov: f64,

    pub width_px: i32, 
    pub height_px: i32,
}

impl Camera {
    pub fn perspective_projection(&self, point: &SpacePoint) -> SpacePoint {
        let camera_transform: SpacePoint = point.subtract(&self.position);

        let camera_coordinate_point = SpacePoint {
            x: (self.focal_length * camera_transform.x) / camera_transform.z,
            y: (self.focal_length * camera_transform.y) / camera_transform.z,
            z: camera_transform.z, // Remember z position
        };

        camera_coordinate_point
    }

    pub fn get_camera_space_dimensions(&self) -> SpacePoint {
        let half_fov_radians: f64 = util::degrees_to_radians(self.vertical_fov / 2.0);
        let height: f64 = 2.0 * self.focal_length * half_fov_radians.tan();
        let width: f64 = height * self.width_px as f64 / self.height_px as f64;

        SpacePoint {
            x: width,
            y: height,
            z: 0.0,
        }
    }

    pub fn get_screen_point(&self, projected_point: &SpacePoint) -> ScreenPoint {
        let camera_space_dimensions: SpacePoint = self.get_camera_space_dimensions();

        let mut screen_point = ScreenPoint {
            x: (projected_point.x / camera_space_dimensions.x * (self.width_px as f64)).round() as i32,
            y: (projected_point.y / camera_space_dimensions.y * (self.height_px as f64)).round() as i32,
        };
        screen_point.x += self.width_px / 2;
        screen_point.y += self.height_px / 2;

        screen_point
    }
}