use std::io::{Error, ErrorKind};

pub struct WichmannHillGenerator {
    seed0: f32,
    seed1: f32,
    seed2: f32,
}

trait Wichmann {
    fn is_valid_wichmann_seed(self) -> bool;
}
impl Wichmann for f32 {
    fn is_valid_wichmann_seed(self) -> bool {
        self >= 1.0 && self <= 30000.0
    }
}

impl WichmannHillGenerator {
    pub fn new(seed0: f32, seed1: f32, seed2: f32) -> Result<WichmannHillGenerator, Error> {
        if !seed0.is_valid_wichmann_seed()
            || !seed1.is_valid_wichmann_seed()
            || !seed2.is_valid_wichmann_seed()
        {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "seeds must be between 1 and 30,000 inclusive",
            ));
        }

        Ok(WichmannHillGenerator {
            seed0,
            seed1,
            seed2,
        })
    }

    pub fn next(&mut self) -> f32 {
        let result = (self.seed0 / 30269.0 + self.seed1 / 30307.0 + self.seed2 / 30323.0) % 1.0;
        // prep for next
        self.seed0 = 171.0 * self.seed0 % 30269.0;
        self.seed1 = 172.0 * self.seed1 % 30307.0;
        self.seed2 = 170.0 * self.seed2 % 30323.0;

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_output() {
        let mut _generator = WichmannHillGenerator::new(1532.0, 24321.0, 3323.0).unwrap();
        // TODO - test output
    }
    #[test]

    fn handle_invalid_input() {
        let mut generator = WichmannHillGenerator::new(0.0, 0.0, 0.0);
        assert!(generator.is_err());
        generator = WichmannHillGenerator::new(30001.0, 0.0, 0.0);
        assert!(generator.is_err());
        generator = WichmannHillGenerator::new(-30001.0, 0.0, 0.0);
        assert!(generator.is_err());
    }
}
