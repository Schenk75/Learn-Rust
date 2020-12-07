// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot{
    position: (i32, i32),
    direction: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Robot { position: (x, y), direction: d }
    }

    pub fn turn_right(self) -> Self {
        let d;
        match self.direction {
            Direction::North => d = Direction::East,
            Direction::South => d = Direction::West,
            Direction::East => d = Direction::South,
            Direction::West => d = Direction::North,
        }

        Robot { position: self.position, direction: d }
    }

    pub fn turn_left(self) -> Self {
        let d;
        match self.direction {
            Direction::North => d = Direction::West,
            Direction::South => d = Direction::East,
            Direction::East => d = Direction::North,
            Direction::West => d = Direction::South,
        }

        Robot { position: self.position, direction: d }
    }

    pub fn advance(self) -> Self {
        let (mut x, mut y) = self.position;
        match self.direction {
            Direction::North => y += 1,
            Direction::South => y -= 1,
            Direction::East => x += 1,
            Direction::West => x -= 1,
        }

        Robot {position: (x, y), direction: self.direction}
    }

    pub fn instructions(self, instructions: &str) -> Self {
        let mut robot = Robot {position: self.position, direction: self.direction};
        for instruction in instructions.chars() {
            match instruction {
                'R' => robot = Self::turn_right(robot),
                'L' => robot = Self::turn_left(robot),
                'A' => robot = Self::advance(robot),
                _ => (),
            }
        }

        robot
    }

    pub fn position(&self) -> (i32, i32) {
        self.position
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}
