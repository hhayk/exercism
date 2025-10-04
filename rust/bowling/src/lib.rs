#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(Default)]
pub struct BowlingGame {
    throws: Vec<u16>,
    second: bool,
}

impl BowlingGame {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        //todo!("Record that {pins} pins have been scored");
        if pins > 10 || (self.second && pins + self.throws.last().unwrap() > 10) {
            Err(Error::NotEnoughPinsLeft)
        } else if self.score().is_some() {
            Err(Error::GameComplete)
        } else {
            self.throws.push(pins);
            self.second = if pins == 10 { false } else { !self.second };
            Ok(())
        }
    }

    pub fn score(&self) -> Option<u16> {
        //todo!("Return the score if the game is complete, or None if not.");
        let mut ans = 0;
        let mut frame = 0;

        for _ in 0..10 {
            if let (Some(&first), Some(&second)) =
                (self.throws.get(frame), self.throws.get(frame + 1))
            {
                ans += first + second;
                if first + second >= 10 {
                    if let Some(&third) = self.throws.get(frame + 2) {
                        ans += third;
                    } else {
                        return None;
                    }
                }
                frame += if first == 10 { 1 } else { 2 };
            } else {
                return None;
            }
        }

        Some(ans)
    }
}
