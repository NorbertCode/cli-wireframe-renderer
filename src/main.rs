mod util;
mod vector3f;
mod shape;
mod camera;
mod terminal_display;

use crossterm::ExecutableCommand;
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
    let camera: Camera = serde_json::from_value(parsed_display_config["camera"].clone())
        .expect("Should be able to parse camera from ./config/display.json");
    let display: TerminalDisplay = serde_json::from_value(parsed_display_config["terminal_display"].clone())
        .expect("Should be able to parse terminal_display from ./config/display.json");

    let mut stdout = std::io::stdout();
    stdout.execute(crossterm::cursor::Hide).unwrap();

    loop {
        for shape in shapes.iter_mut() {
            shape.rotate(&Vector3f { x: 1.0, y: 1.0, z: 1.0 });
        }
        display.display_loop_iteration(&mut shapes, &camera, &mut stdout);
    }
}
