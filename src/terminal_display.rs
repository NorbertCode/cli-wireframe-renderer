use crate::camera::Camera;
use crate::point::SpacePoint;
use crate::shape::Shape;

#[derive(PartialEq, Eq)]
pub struct ScreenPoint {
    pub x: i32,
    pub y: i32,
    pub z_index: i32,
}

pub struct TerminalDisplay {
    pub width: i32,
    pub height: i32,
    pub frame_time_millis: u64,
    
    pub vertex_char: char,
}

impl TerminalDisplay {
    pub fn display_loop(&self, shapes: &mut Vec<Shape>, camera: &Camera) {
        loop {
            TerminalDisplay::print_display(self.draw_shapes(&shapes, &camera));
            for shape in shapes.iter_mut() {
                shape.rotate(1.0);
            }
            std::thread::sleep(std::time::Duration::from_millis(self.frame_time_millis));
            print!("\x1B[2J\x1B[1;1H");
        }
    } 

    pub fn draw_shapes(&self, shapes: &Vec<Shape>, camera: &Camera) -> Vec<Vec<char>> {
        let mut display = vec![vec![' '; self.width as usize]; self.height as usize];

        let camera_space_dimensions = camera.get_camera_space_dimensions();
        for shape in shapes {
            for point in &shape.points {
                let screen_point: ScreenPoint = self.get_screen_point(&camera.perspective_projection(&point), camera_space_dimensions);
                display[screen_point.y as usize][screen_point.x as usize] = self.vertex_char;
            }
        }

        display
    }

    fn print_display(display_vector: Vec<Vec<char>>) {
        for row in display_vector {
            for chr in row {
                print!("{}", chr);
            }
            print!("\n");
        }
    }

    fn get_screen_point(&self, projected_point: &SpacePoint, camera_space_dimensions: (f64, f64)) -> ScreenPoint {
        let mut screen_point = ScreenPoint {
            x: (projected_point.x / camera_space_dimensions.0 * (self.width as f64)).round() as i32,
            y: (projected_point.y / camera_space_dimensions.1 * (self.height as f64)).round() as i32,
            z_index: projected_point.z.round() as i32,
        };
        screen_point.x += self.width / 2;
        screen_point.y += self.height / 2;

        screen_point
    }
}