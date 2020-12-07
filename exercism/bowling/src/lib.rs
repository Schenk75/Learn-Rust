#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    current_roll: u8,
    current_frame: u8,
    roll_index: usize,
    rolls: [u16; 21],
    is_completed: bool,
}

impl Default for BowlingGame {
    fn default() -> Self {
        BowlingGame::new()
    }
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame {
            current_roll: 0,
            current_frame: 0,
            roll_index: 0,
            rolls: [0; 21],
            is_completed: false,
        }
    }

    // current_roll -> 0, 1(, 2 only in 9th i.e. last Frame)
    // current_frame -> 0-9
    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if pins > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }

        if self.is_completed {
            return Err(Error::GameComplete);
        }

        if self.current_frame == 9 {
            if self.current_roll == 1 {
                if !(self.rolls[self.roll_index - 1] == 10
                    || self.rolls[self.roll_index - 1] + pins == 10)
                {
                    self.is_completed = true;
                }
                if self.rolls[self.roll_index - 1] != 10
                    && pins + self.rolls[self.roll_index - 1] > 10
                {
                    return Err(Error::NotEnoughPinsLeft);
                }
            } else if self.current_roll == 2 {
                self.is_completed = true;
                if self.rolls[self.roll_index - 1] != 10
                    && (self.rolls[self.roll_index - 1] + self.rolls[self.roll_index - 2] != 10)
                    && self.rolls[self.roll_index - 1] + pins > 10
                {
                    return Err(Error::NotEnoughPinsLeft);
                }
            }
            self.current_roll += 1;
        } else {
            match self.current_roll {
                0 => {
                    if pins == 10 {
                        self.current_roll = 0;
                        self.current_frame += 1;
                    } else {
                        self.current_roll = 1;
                    }
                }
                1 => {
                    if pins + self.rolls[self.roll_index - 1] > 10 {
                        return Err(Error::NotEnoughPinsLeft);
                    }
                    self.current_roll = 0;
                    self.current_frame += 1;
                }
                _ => {}
            }
        }
        self.rolls[self.roll_index] = pins;
        self.roll_index += 1;
        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        if !self.is_completed {
            return None;
        }
        let mut total_score = 0;
        let mut roll_index = 0;
        for _frame in 0..10 {
            if self.rolls[roll_index] == 10 {
                total_score += self.rolls[roll_index]
                    + self.rolls[roll_index + 1]
                    + self.rolls[roll_index + 2];
                roll_index += 1;
            } else if (self.rolls[roll_index] + self.rolls[roll_index + 1]) == 10 {
                total_score += self.rolls[roll_index]
                    + self.rolls[roll_index + 1]
                    + self.rolls[roll_index + 2];
                roll_index += 2;
            } else {
                total_score += self.rolls[roll_index] + self.rolls[roll_index + 1];
                roll_index += 2;
            }
        }
        Some(total_score)
    }
}