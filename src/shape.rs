use crate::vector3f::Vector3f;

pub struct Shape {
    pub points: Vec<Vector3f>,
    pub edges: Vec<(usize, usize)>, // Pairs of indexes of points
    pub origin: Vector3f,
}

impl Shape {
    pub fn rotate(&mut self, angle_degrees_clockwise: &Vector3f) {
        for point in &mut self.points.iter_mut() {
            *point = point.rotate_point(&self.origin, &angle_degrees_clockwise);
        }
    }
}