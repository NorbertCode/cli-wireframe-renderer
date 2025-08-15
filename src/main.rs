struct SpacePoint {
    x: f64,
    y: f64,
    z: f64,
}

struct Camera { 
    // By default at position (0,0,0) and rotation (0,0,0)
    focal_length: f64,
}

fn perspective_projection(point: &SpacePoint, camera: &Camera) -> SpacePoint {
    let camera_coordinate_point = SpacePoint {
        x: (camera.focal_length * point.x) / point.z,
        y: (camera.focal_length * point.y) / point.z,
        z: point.z,
    };

    camera_coordinate_point
}

fn main() {
    let camera = Camera {
        focal_length: 1.0,
    };

    let point1 = SpacePoint {x: 1.0, y: 3.0, z: 3.0};
    let screen_point1: SpacePoint = perspective_projection(&point1, &camera);

    let point2 = SpacePoint {x: 1.0, y: 0.0, z: 3.0};
    let screen_point2: SpacePoint = perspective_projection(&point2, &camera);

    let point3 = SpacePoint {x: 1.0, y: 3.0, z: 6.0};
    let screen_point3: SpacePoint = perspective_projection(&point3, &camera);

    let point4 = SpacePoint {x: 1.0, y: 0.0, z: 6.0};
    let screen_point4: SpacePoint = perspective_projection(&point4, &camera);

    println!("({}, {})", screen_point1.x, screen_point1.y);
    println!("({}, {})", screen_point2.x, screen_point2.y);
    println!("({}, {})", screen_point3.x, screen_point3.y);
    println!("({}, {})", screen_point4.x, screen_point4.y);
}
