mod util;
mod vector3f;
mod shape;
mod camera;
mod terminal_display;

use std::{io, panic, sync::mpsc, thread, time::{Duration, Instant}};

use crossterm::{event::{self, Event, KeyCode, poll, read}, terminal};
use vector3f::Vector3f;
use shape::Shape;
use camera::Camera;
use terminal_display::TerminalDisplay;

fn setup_failsafe() {
    let original_hook = panic::take_hook();

    panic::set_hook(Box::new(move |panic_info| {
            let _ = terminal::disable_raw_mode();
            original_hook(panic_info);
        }
    ));
}

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

    let target_frame_time = Duration::from_millis(16); // 60 fps

    let mut stdout = std::io::stdout();
    let (tx, rx) = mpsc::channel();

    setup_failsafe();

    crossterm::terminal::enable_raw_mode().expect("Failed to enter raw mode");

    thread::spawn(move || {
        loop {
            if event::poll(Duration::from_millis(50)).unwrap() {
                if let Event::Key(key_event) = event::read().unwrap() {
                    tx.send(key_event).unwrap();
                }
            }
        }
    });

    'core_loop: loop {
        let frame_start = Instant::now();

        let mut camera_position_transform = Vector3f { x: 0.0, y: 0.0, z: 0.0 };
        let mut camera_rotation_transform = Vector3f { x: 0.0, y: 0.0, z: 0.0 };

        while let Ok(key_event) = rx.try_recv() {
            match key_event.code {
                KeyCode::Char('w') => camera_position_transform.z += 1.0,
                KeyCode::Char('s') => camera_position_transform.z -= 1.0,
                KeyCode::Char('a') => camera_position_transform.x -= 1.0,
                KeyCode::Char('d') => camera_position_transform.x += 1.0,
                KeyCode::Char('q') => camera_position_transform.y -= 1.0,
                KeyCode::Char('e') => camera_position_transform.y += 1.0,
                KeyCode::Up => camera_rotation_transform.x += 5.0,
                KeyCode::Down => camera_rotation_transform.x -= 5.0,
                KeyCode::Left => camera_rotation_transform.y -= 5.0,
                KeyCode::Right => camera_rotation_transform.y += 5.0,
                KeyCode::Esc => break 'core_loop,
                _ => {}
            }
        }

        camera_position_transform = camera_position_transform.rotate_point_around_origin(&camera.rotation);
        camera.position = camera.position.add(&camera_position_transform);
        camera.rotation = camera.rotation.add(&camera_rotation_transform);

        display.display_loop_iteration(&mut shapes, &camera, &mut stdout);

        let elapsed = frame_start.elapsed();
        if elapsed < target_frame_time {
            thread::sleep(target_frame_time - elapsed);
        }
    }

    crossterm::terminal::disable_raw_mode().expect("Failed to disable raw mode");
}
