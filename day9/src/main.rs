use day9::{
    direction::Direction,
    input::{input, input_test},
    snake::Snake,
};

fn main() {
    let mut day9 = Day9::default();
    input().iter().for_each(|f| {
        let (direction, distance) = f.split_once(' ').unwrap();
        day9.steps
            .push((Direction::parse(direction), distance.parse().unwrap()));
    });

    dbg!(day9.part1());
}

#[derive(Debug, Default)]
pub struct Day9 {
    steps: Vec<(Direction, i32)>,
}

impl Day9 {
    fn part1(&mut self) -> usize {
        let mut snake = Snake::default();

        self.steps.iter().for_each(|(direction, distance)| {
            (0..*distance).for_each(|_| {
                snake.do_move(direction);
            });
        });
        snake.visited.len()
    }
}
