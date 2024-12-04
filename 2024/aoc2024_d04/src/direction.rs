#[derive(Copy, Clone)]
pub enum Direction {
    Top,
    TopRight,
    Right,
    BottomRight,
    Bottom,
    BottomLeft,
    Left,
    TopLeft,
}

impl Direction {
    pub fn get_all_directions() -> Vec<Direction> {
        vec![
            Direction::Top,
            Direction::TopRight,
            Direction::Right,
            Direction::BottomRight,
            Direction::Bottom,
            Direction::BottomLeft,
            Direction::Left,
            Direction::TopLeft,
        ]
    }

    pub fn is_valid(&self, grid: &Vec<Vec<char>>, x: usize, y: usize, len: usize) -> bool {
        let target_length = len - 1; // adjust for 0 indexing
        let valid_top = x >= target_length;
        let valid_right = y < grid[0].len() - target_length;
        let valid_left = y >= target_length;
        let valid_bottom = x < grid.len() - target_length;

        match self {
            Direction::Top => valid_top,
            Direction::TopRight => valid_top && valid_right,
            Direction::TopLeft => valid_top && valid_left,
            Direction::Left => valid_left,
            Direction::Right => valid_right,
            Direction::Bottom => valid_bottom,
            Direction::BottomRight => valid_bottom && valid_right,
            Direction::BottomLeft => valid_bottom && valid_left,
        }
    }
}
