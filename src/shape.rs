use serde::Deserialize;

use crate::vector3f::Vector3f;

#[derive(Deserialize)]
pub struct Edge {
    pub start: usize,
    pub end: usize,
}

#[derive(Deserialize)]
pub struct Shape {
    pub points: Vec<Vector3f>,
    pub edges: Vec<Edge>, // Pairs of indexes of points
    pub origin: Vector3f,

    pub velocity: Vector3f,
    pub rotation_velocity: Vector3f,
}

impl Shape {
    pub fn update(&mut self, delta_time: f64) {
        let current_vel = self.velocity.multiply(delta_time);
        let current_rot_vel = self.rotation_velocity.multiply(delta_time);

        self.translate(&current_vel);
        self.rotate(&current_rot_vel);
    }

    pub fn translate(&mut self, translation: &Vector3f) {
        for point in &mut self.points.iter_mut() {
            *point = point.add(&translation);
        }
    }

    pub fn rotate(&mut self, angle_degrees_clockwise: &Vector3f) {
        for point in &mut self.points.iter_mut() {
            *point = point.rotate_point(&self.origin, &angle_degrees_clockwise);
        }
    }
}