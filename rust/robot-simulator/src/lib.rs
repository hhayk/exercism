// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Eq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    x: i32,
    y: i32,
    d: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        // todo!("Create a robot at (x, y) ({x}, {y}) facing {d:?}")
        Self { x, y, d }
    }

    #[must_use]
    pub fn turn_right(self) -> Self {
        match self.d {
            Direction::North => Self {
                x: self.x,
                y: self.y,
                d: Direction::East,
            },
            Direction::East => Self {
                x: self.x,
                y: self.y,
                d: Direction::South,
            },
            Direction::South => Self {
                x: self.x,
                y: self.y,
                d: Direction::West,
            },
            Direction::West => Self {
                x: self.x,
                y: self.y,
                d: Direction::North,
            },
        }
    }

    #[must_use]
    pub fn turn_left(self) -> Self {
        match self.d {
            Direction::North => Self {
                x: self.x,
                y: self.y,
                d: Direction::West,
            },
            Direction::East => Self {
                x: self.x,
                y: self.y,
                d: Direction::North,
            },
            Direction::South => Self {
                x: self.x,
                y: self.y,
                d: Direction::East,
            },
            Direction::West => Self {
                x: self.x,
                y: self.y,
                d: Direction::South,
            },
        }
    }

    #[must_use]
    pub fn advance(self) -> Self {
        match self.d {
            Direction::North => Self {
                x: self.x,
                y: self.y + 1,
                d: self.d,
            },
            Direction::East => Self {
                x: self.x + 1,
                y: self.y,
                d: self.d,
            },
            Direction::South => Self {
                x: self.x,
                y: self.y - 1,
                d: self.d,
            },
            Direction::West => Self {
                x: self.x - 1,
                y: self.y,
                d: self.d,
            },
        }
    }

    #[must_use]
    pub fn instructions(self, instructions: &str) -> Self {
        // todo!("Follow the given sequence of instructions: {instructions}")
        instructions.chars().fold(self, |acc, ch| match ch {
            'R' => acc.turn_right(),
            'L' => acc.turn_left(),
            'A' => acc.advance(),
            _ => unreachable!(),
        })
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.d
    }
}
