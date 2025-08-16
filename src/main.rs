fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * std::f64::consts::PI / 180.0
}

struct SpacePoint {
    x: f64,
    y: f64,
    z: f64,
}

fn negate_point(point: &SpacePoint) -> SpacePoint {
    SpacePoint {
        x: -point.x,
        y: -point.y,
        z: -point.z,
    }
}

fn add_points(a: &SpacePoint, b: &SpacePoint) -> SpacePoint {
    SpacePoint {
        x: a.x - b.x,
        y: a.y + b.y,
        z: a.z + b.z,
    }
}

fn subtract_points(a: &SpacePoint, b: &SpacePoint) -> SpacePoint {
    add_points(&a, &negate_point(&b))
}

fn rotate_point_around_origin(point: &SpacePoint, angle_degrees_clockwise: f64) -> SpacePoint {
    let angle_radians: f64 = degrees_to_radians(-angle_degrees_clockwise);
    let angle_sin: f64 = angle_radians.sin();
    let angle_cos: f64 = angle_radians.cos();

    SpacePoint {
        x: (point.x * angle_cos) + (point.z * angle_sin),
        y: point.y,
        z: (-point.x * angle_sin) + (point.z * angle_cos),
    }
}

fn rotate_point(point: &SpacePoint, origin: &SpacePoint, angle_degrees_clockwise: f64) -> SpacePoint {
    let mut centered_point: SpacePoint = subtract_points(point, origin);
    centered_point = rotate_point_around_origin(&centered_point, angle_degrees_clockwise);

    add_points(&centered_point, &origin)
}

struct Camera { 
    // By default at rotation (0,0,0)
    position: SpacePoint,
    focal_length: f64,
    vertical_fov: f64,

    width_px: i32, 
    height_px: i32,
}

#[derive(PartialEq, Eq)]
struct ScreenPoint {
    x: i32,
    y: i32,
}

fn perspective_projection(point: &SpacePoint, camera: &Camera) -> SpacePoint {
    let camera_transform: SpacePoint = subtract_points(point, &camera.position);

    let camera_coordinate_point = SpacePoint {
        x: (camera.focal_length * camera_transform.x) / camera_transform.z,
        y: (camera.focal_length * camera_transform.y) / camera_transform.z,
        z: camera_transform.z, // Remember z position
    };

    camera_coordinate_point
}

fn get_camera_space_dimensions(camera: &Camera) -> SpacePoint {
    let half_fov_radians: f64 = degrees_to_radians(camera.vertical_fov / 2.0);
    let height: f64 = 2.0 * camera.focal_length * half_fov_radians.tan();
    let width: f64 = height * camera.width_px as f64 / camera.height_px as f64;

    SpacePoint {
        x: width,
        y: height,
        z: 0.0,
    }
}

fn get_screen_point(projected_point: &SpacePoint, camera: &Camera) -> ScreenPoint {
    let camera_space_dimensions: SpacePoint = get_camera_space_dimensions(&camera);

    let mut screen_point = ScreenPoint {
        x: (projected_point.x / camera_space_dimensions.x * (camera.width_px as f64)).round() as i32,
        y: (projected_point.y / camera_space_dimensions.y * (camera.height_px as f64)).round() as i32,
    };
    screen_point.x += camera.width_px / 2;
    screen_point.y += camera.height_px / 2;

    screen_point
}

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
            let projected_point: SpacePoint = perspective_projection(&rotate_point(&points[i], &rotation_origin, degrees), &camera);
            screen_points.push(get_screen_point(&projected_point, &camera));
        }

        for y in 0..camera.height_px + 2 {
            for x in 0..camera.width_px + 2 {
                if x == 0 || x == camera.width_px + 1 || y == 0 || y == camera.height_px + 1 {
                    print!("#");
                    continue;
                }

                let this_point = ScreenPoint { x: x, y: y };
                if screen_points.contains(&this_point) {
                    print!("-");
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
    }
}
