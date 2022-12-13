#[derive(Debug, Default)]
pub enum Direction {
    #[default]
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn parse(s: &str) -> Self {
        match s {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => unreachable!(),
        }
    }

    pub fn movement_tuple(&self) -> (i32, i32) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
    }
}
