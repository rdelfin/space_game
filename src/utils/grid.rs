use ggez::nalgebra::{Matrix2, Point2, Point3, Vector2};

pub const TILE_WIDTH: i32 = 104;
pub const TILE_HEIGHT: i32 = 120;

pub fn grid_to_position(grid_pos: Point2<i32>) -> Point2<f32> {
    let system_base = Point2::new(TILE_WIDTH as f32 / 2.0, TILE_HEIGHT as f32 / 2.0);
    let (w, h) = (TILE_WIDTH as f32, TILE_HEIGHT as f32);
    let coord_matrix = Matrix2::new(w, w / 2.0, 0.0, 3.0 * h / 4.0);

    system_base + coord_matrix * Vector2::new(grid_pos.x as f32, grid_pos.y as f32)
}

// Return the nearest grid coordinates
pub fn position_to_grid(pos: Point2<f32>) -> Point2<i32> {
    let system_base = Point2::new(TILE_WIDTH as f32 / 2.0, TILE_HEIGHT as f32 / 2.0);
    let (w, h) = (TILE_WIDTH as f32, TILE_HEIGHT as f32);
    let coord_matrix = Matrix2::new(1.0 / w, -2.0 / (3.0 * h), 0.0, 4.0 / (3.0 * h));

    let frac_coord = coord_matrix * (pos - system_base);
    let rounded_coord = cube_to_axial(cube_round(axial_to_cube(Point2::new(
        frac_coord.x,
        frac_coord.y,
    ))));
    Point2::new(rounded_coord.x as i32, rounded_coord.y as i32)
}

pub fn axial_to_cube(axial: Point2<f32>) -> Point3<f32> {
    Point3::new(axial.x, -(axial.x + axial.y), axial.y)
}

pub fn cube_to_axial(cube: Point3<f32>) -> Point2<f32> {
    Point2::new(cube.x, cube.z)
}

pub fn cube_round(pos: Point3<f32>) -> Point3<f32> {
    let mut rx = (pos.x).round();
    let mut ry = (pos.y).round();
    let mut rz = (pos.z).round();

    let x_diff = (rx - pos.x).abs();
    let y_diff = (ry - pos.y).abs();
    let z_diff = (rz - pos.z).abs();

    if x_diff > y_diff && x_diff > z_diff {
        rx = -ry - rz;
    } else if y_diff > z_diff {
        ry = -rx - rz;
    } else {
        rz = -rx - ry;
    }

    Point3::new(rx, ry, rz)
}

#[derive(Debug, Clone)]
pub enum WallDirection {
    Right,
    TopRight,
    TopLeft,
    Left,
    BottomLeft,
    BottomRight,
}

pub fn wall_direction_to_offset(dir: WallDirection) -> Vector2<i32> {
    match dir {
        WallDirection::Right => Vector2::new(1, 0),
        WallDirection::TopRight => Vector2::new(1, 1),
        WallDirection::TopLeft => Vector2::new(0, -1),
        WallDirection::Left => Vector2::new(-1, 0),
        WallDirection::BottomLeft => Vector2::new(-1, 1),
        WallDirection::BottomRight => Vector2::new(0, 1),
    }
}

// The ordering might be important here for a given use, so they're provided starting on the right
// going around clockwise.
pub fn neighbours(pos: Point2<i32>) -> [Point2<i32>; 6] {
    [
        pos + Vector2::new(1, 0),
        pos + Vector2::new(1, -1),
        pos + Vector2::new(0, -1),
        pos + Vector2::new(-1, 0),
        pos + Vector2::new(-1, 1),
        pos + Vector2::new(0, 1),
    ]
}
