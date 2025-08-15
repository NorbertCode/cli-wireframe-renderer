struct SpacePoint {
    x: f64,
    y: f64,
    z: f64,
}

struct Camera { 
    // By default at rotation (0,0,0)
    position: SpacePoint,
    focal_length: f64,
}

fn perspective_projection(point: &SpacePoint, camera: &Camera) -> SpacePoint {
    let camera_transform = SpacePoint {
        x: point.x - camera.position.x,
        y: point.y - camera.position.y,
        z: point.z - camera.position.z
    };

    let camera_coordinate_point = SpacePoint {
        x: (camera.focal_length * camera_transform.x) / camera_transform.z,
        y: (camera.focal_length * camera_transform.y) / camera_transform.z,
        z: camera_transform.z,
    };

    camera_coordinate_point
}

fn main() {
    let camera = Camera {
        position: SpacePoint { x: 0.0, y: 0.0, z: 0.0 },
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
