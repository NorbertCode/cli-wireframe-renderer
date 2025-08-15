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
    let angle_radians: f64 = -angle_degrees_clockwise * std::f64::consts::PI / 180.0;
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

fn main() {
    let camera = Camera {
        position: SpacePoint { x: 0.0, y: 0.0, z: -5.0 },
        focal_length: 1.0,
    };

    let point1 = SpacePoint {x: 1.0, y: 3.0, z: 3.0};
    let screen_point1: SpacePoint = perspective_projection(&point1, &camera);
    let rotated_point1: SpacePoint = rotate_point_around_origin(&point1, -45.0);
    let rotated_screen_point1: SpacePoint = perspective_projection(&rotated_point1, &camera);

    println!("({}, {})", screen_point1.x, screen_point1.y);
    println!("({}, {}, {})", rotated_point1.x, rotated_point1.y, rotated_point1.z);
    println!("({}, {})", rotated_screen_point1.x, rotated_screen_point1.y)
}
