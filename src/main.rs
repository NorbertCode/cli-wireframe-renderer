mod util;
mod point;
mod shape;
mod camera;
mod terminal_display;

use point::SpacePoint;
use shape::Shape;
use camera::Camera;

use crate::terminal_display::TerminalDisplay;

fn main() {
    let rect = Shape {
        points: vec![
            SpacePoint { x: 3.0, y: 3.0, z: 8.0},
            SpacePoint { x: 3.0, y: 3.0, z: 2.0},
            SpacePoint { x: 3.0, y: -3.0, z: 8.0},
            SpacePoint { x: 3.0, y: -3.0, z: 2.0},
            SpacePoint { x: -3.0, y: 3.0, z: 8.0},
            SpacePoint { x: -3.0, y: 3.0, z: 2.0},
            SpacePoint { x: -3.0, y: -3.0, z: 8.0},
            SpacePoint { x: -3.0, y: -3.0, z: 2.0},
        ],
        origin: SpacePoint { x: 0.0, y: 0.0, z: 5.0 },
    };
    let mut shapes = vec![rect];

    let camera = Camera {
        position: SpacePoint { x: 0.0, y: 0.0, z: -5.0 },
        focal_length: 1.0,
        vertical_fov: 60.0,
        aspect_ratio: 2.0,
    };

    let display = TerminalDisplay {
        width: 64,
        height: 32,
        frame_time_millis: 10,
        vertex_char: 'O',
    };

    display.display_loop(&mut shapes, &camera);
}
