mod util;
mod point;
mod camera;

use point::SpacePoint;
use camera::Camera;
use camera::ScreenPoint;


fn main() {
    let camera = Camera {
        position: SpacePoint { x: 0.0, y: 0.0, z: -5.0 },
        focal_length: 1.0,
        vertical_fov: 60.0,
        width_px: 64,
        height_px: 32,
    };

    let rotation_origin = SpacePoint {
        x: 0.0,
        y: 0.0,
        z: 5.0,
    };

    let points: Vec<SpacePoint> = vec![
        SpacePoint { x: 3.0, y: 3.0, z: 8.0},
        SpacePoint { x: 3.0, y: 3.0, z: 2.0},
        SpacePoint { x: 3.0, y: -3.0, z: 8.0},
        SpacePoint { x: 3.0, y: -3.0, z: 2.0},
        SpacePoint { x: -3.0, y: 3.0, z: 8.0},
        SpacePoint { x: -3.0, y: 3.0, z: 2.0},
        SpacePoint { x: -3.0, y: -3.0, z: 8.0},
        SpacePoint { x: -3.0, y: -3.0, z: 2.0},
    ];
    let mut screen_points: Vec<ScreenPoint> = vec![];
    let mut degrees: f64 = 0.0;

    loop {
        screen_points.clear();
        for i in 0..points.len() {
            let projected_point: SpacePoint = camera.perspective_projection(&points[i].rotate_point(&rotation_origin, degrees));
            screen_points.push(camera.get_screen_point(&projected_point));
        }

        for y in 0..camera.height_px + 2 {
            for x in 0..camera.width_px + 2 {
                if x == 0 || x == camera.width_px + 1 || y == 0 || y == camera.height_px + 1 {
                    print!("#");
                    continue;
                }

                let this_point = ScreenPoint { x: x, y: y };
                if screen_points.contains(&this_point) {
                    print!("O");
                }
                else {
                    print!(" ");
                }
            }
            print!("\n");
        }
        
        std::thread::sleep(std::time::Duration::from_millis(10));
        print!("\x1B[2J\x1B[1;1H");

        degrees += 1.0;
        if degrees >= 360.0 {
            degrees = 0.0;
        }
        else if degrees <= 0.0 {
            degrees = 360.0;
        }
    }
}
