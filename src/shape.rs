use crate::point::SpacePoint;

pub struct Shape {
    pub points: Vec<SpacePoint>,
    pub edges: Vec<(usize, usize)>, // Pairs of indexes of points
    pub origin: SpacePoint,
}

impl Shape {
    pub fn rotate(&mut self, angle_degrees_clockwise: (f64, f64, f64)) {
        for point in &mut self.points.iter_mut() {
            *point = point.rotate_point(&self.origin, angle_degrees_clockwise);
        }
    }
}