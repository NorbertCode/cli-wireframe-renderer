mod util;
mod vector3f;
mod shape;
mod camera;
mod terminal_display;

use std::{io, sync::mpsc, thread, time::Duration};

use crossterm::event::{self, poll, read, Event, KeyCode};
use vector3f::Vector3f;
use shape::Shape;
use camera::Camera;
use terminal_display::TerminalDisplay;

fn main() {
    const SHAPES_CONFIG_PATH: &str = "./config/shapes.json";
    const DISPLAY_CONFIG_PATH: &str = "./config/display.json";

    let shapes_config: String = std::fs::read_to_string(SHAPES_CONFIG_PATH)
        .expect("Should be able to read ./config/shapes.json");
    let display_config: String = std::fs::read_to_string(DISPLAY_CONFIG_PATH)
        .expect("Should be able to read ./config/display.json");
    let parsed_display_config: serde_json::Value = serde_json::from_str(&display_config)
        .expect("Should be able to pare ./config/display.json");

    let mut shapes: Vec<Shape> = serde_json::from_str(&shapes_config)
        .expect("Should be able to parse ./config/shapes.json");
    let mut camera: Camera = serde_json::from_value(parsed_display_config["camera"].clone())
        .expect("Should be able to parse camera from ./config/display.json");
    let mut display: TerminalDisplay = serde_json::from_value(parsed_display_config["terminal_display"].clone())
        .expect("Should be able to parse terminal_display from ./config/display.json");

    display.width = crossterm::terminal::window_size().expect("Can't read window size").columns as i32;
    display.height = crossterm::terminal::window_size().expect("Can't read window size").rows as i32;
    camera.aspect_ratio = display.width as f64 / display.height as f64;

    let mut stdout = std::io::stdout();
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        loop {
            crossterm::terminal::enable_raw_mode();
            if event::poll(Duration::from_millis(500)).unwrap() {
                if let Event::Key(key_event) = event::read().unwrap() {
                    tx.send(key_event).unwrap();
                }
            }
        }
    });

    loop {
        let mut camera_position_transform = Vector3f { x: 0.0, y: 0.0, z: 0.0 };
        let mut camera_rotation_transform = Vector3f { x: 0.0, y: 0.0, z: 0.0 };

        if let Ok(key_event) = rx.try_recv() {
            match key_event.code {
                KeyCode::Char('w') => {
                    camera_position_transform.z += 1.0;
                },
                KeyCode::Char('s') => {
                    camera_position_transform.z -= 1.0;
                },
                KeyCode::Char('a') => {
                    camera_position_transform.x -= 1.0;
                },
                KeyCode::Char('d') => {
                    camera_position_transform.x += 1.0;
                },
                KeyCode::Char('q') => {
                    camera_position_transform.y -= 1.0;
                },
                KeyCode::Char('e') => {
                    camera_position_transform.y += 1.0;
                },
                KeyCode::Up => {
                    camera_rotation_transform.x += 5.0;
                }
                KeyCode::Down => {
                    camera_rotation_transform.x -= 5.0;
                }
                KeyCode::Left => {
                    camera_rotation_transform.y -= 5.0;
                }
                KeyCode::Right => {
                    camera_rotation_transform.y += 5.0;
                }
                _ => {}
            }
        }

        camera_position_transform = camera_position_transform.rotate_point_around_origin(&camera.rotation);
        camera.position = camera.position.add(&camera_position_transform);
        camera.rotation = camera.rotation.add(&camera_rotation_transform);

        crossterm::terminal::disable_raw_mode().unwrap();
        display.display_loop_iteration(&mut shapes, &camera, &mut stdout);
    }
}
