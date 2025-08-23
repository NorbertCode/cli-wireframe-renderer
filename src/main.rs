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
        edges: vec![
            (0, 1), (0, 2), (0, 4),
            (1, 3), (1, 5),
            (2, 3), (2, 6),
            (3, 7),
            (4, 5), (4, 6),
            (5, 7),
            (6, 7),
        ],
        origin: SpacePoint { x: 0.0, y: 0.0, z: 5.0 },
    };
    let mut shapes = vec![rect];

    let camera = Camera {
        position: SpacePoint { x: 0.0, y: 0.0, z: -6.0 },
        focal_length: 1.0,
        vertical_fov: 60.0,
        aspect_ratio: 2.0,
    };

    let display = TerminalDisplay {
        width: 96,
        height: 48,
        frame_time_millis: 10,
        edge_char: '*',
    };

    loop {
        for shape in shapes.iter_mut() {
            shape.rotate((1.0, 1.0, 1.0));
        }
        display.display_loop_iteration(&mut shapes, &camera);
    }
}
