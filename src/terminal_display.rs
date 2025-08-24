use crate::camera::Camera;
use crate::vector3f::Vector3f;
use crate::shape::Shape;

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct ScreenPoint {
    pub x: i32,
    pub y: i32,
}

pub struct TerminalDisplay {
    pub width: i32,
    pub height: i32,
    pub frame_time_millis: u64,
    
    pub edge_char: char,
}

impl TerminalDisplay {
    pub fn display_loop_iteration(&self, shapes: &mut Vec<Shape>, camera: &Camera) {
        TerminalDisplay::print_display(self.draw_shapes(&shapes, &camera));
        std::thread::sleep(std::time::Duration::from_millis(self.frame_time_millis));
        print!("\x1B[2J\x1B[1;1H");
    } 

    pub fn draw_shapes(&self, shapes: &Vec<Shape>, camera: &Camera) -> Vec<Vec<char>> {
        let mut display = vec![vec![' '; self.width as usize]; self.height as usize];

        let edge_points: Vec<ScreenPoint> = self.get_edge_screen_points(shapes, camera);
        for point in edge_points {
            display[point.y as usize][point.x as usize] = self.edge_char;
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

    fn get_edge_screen_points(&self, shapes: &Vec<Shape>, camera: &Camera) -> Vec<ScreenPoint> {
        let mut points_to_draw: Vec<ScreenPoint> = vec![];
        let camera_space_dimensions = camera.get_camera_space_dimensions();

        for shape in shapes {
            let mut vertices: Vec<ScreenPoint> = vec![];
            for point in &shape.points {
                vertices.push(self.get_screen_point(&camera.perspective_projection(&point), camera_space_dimensions));
            }

            for edge in &shape.edges {
                let dx: i32 = (vertices[edge.1].x - vertices[edge.0].x).abs();
                let dy: i32 = (vertices[edge.1].y - vertices[edge.0].y).abs();

                let sx: i32 = if vertices[edge.0].x < vertices[edge.1].x { 1 } else { -1 };
                let sy: i32 = if vertices[edge.0].y < vertices[edge.1].y { 1 } else { -1 };

                let mut err: i32 = dx - dy;

                let mut x: i32 = vertices[edge.0].x;
                let mut y: i32 = vertices[edge.0].y;

                // Weird workaround to check if it actually reaches the endpoint
                let mut updated_dx: i32 = dx;
                let mut prev_dx: i32;
                loop {
                    prev_dx = updated_dx;
                    updated_dx = (vertices[edge.1].x - x).abs(); 
                    if (x == vertices[edge.1].x && y == vertices[edge.1].y) || updated_dx > prev_dx || x < 0 || x >= self.width || y < 0 || y >= self.height {
                        break;
                    }

                    points_to_draw.push(ScreenPoint { x, y });

                    if 2 * err > -dy {
                        err -= dy;
                        x += sx;
                    }
                    if 2 * err < dx {
                        err += dx;
                        y += sy;
                    }
                }
            }
        }

        points_to_draw
    }

    fn get_screen_point(&self, projected_point: &Vector3f, camera_space_dimensions: (f64, f64)) -> ScreenPoint {
        let mut screen_point = ScreenPoint {
            x: (projected_point.x / camera_space_dimensions.0 * (self.width as f64)).round() as i32,
            y: (projected_point.y / camera_space_dimensions.1 * (self.height as f64)).round() as i32,
        };
        screen_point.x += self.width / 2;
        screen_point.y += self.height / 2;

        screen_point
    }
}