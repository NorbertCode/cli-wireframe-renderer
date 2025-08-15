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
        width_px: 32,
        height_px: 32,
    };

    println!("{} x {}", get_camera_space_dimensions(&camera).x, get_camera_space_dimensions(&camera).y);
    
    let projected_point = SpacePoint {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    println!("({}, {})", get_screen_point(&projected_point, &camera).x, get_screen_point(&projected_point, &camera).y);
    // let point1 = SpacePoint {x: 1.0, y: 3.0, z: 3.0};
    // let projected_point1: SpacePoint = perspective_projection(&point1, &camera);
    // let rotated_point1: SpacePoint = rotate_point_around_origin(&point1, -45.0);
    // let rotated_projected_point1: SpacePoint = perspective_projection(&rotated_point1, &camera);
    // let screen_point1: ScreenPoint = get_screen_point(&projected_point1, &camera);
    // let rotated_screen_point1: ScreenPoint = get_screen_point(&rotated_projected_point1, &camera);

    // println!("({}, {})", projected_point1.x, projected_point1.y);
    // println!("({}, {}, {})", rotated_point1.x, rotated_point1.y, rotated_point1.z);
    // println!("({}, {})", rotated_projected_point1.x, rotated_projected_point1.y);
    // println!("({}, {})", screen_point1.x, screen_point1.y);
    // println!("({}, {})", rotated_screen_point1.x, rotated_screen_point1.y);
}
