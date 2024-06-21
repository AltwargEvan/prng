use std::io::{Error, ErrorKind};

pub struct LinearFeedbackShiftRegistry {
    state: u32,
    //  https://stackoverflow.com/questions/44584215/linear-feedback-shift-register-explaination
    toggle_mask: u32,
}

impl LinearFeedbackShiftRegistry {
    pub fn new(seed: u32, toggle_mask: u32) -> Result<LinearFeedbackShiftRegistry, Error> {
        if seed == 0 {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "seed must be greater than 0",
            ));
        }

        Ok(LinearFeedbackShiftRegistry {
            state: seed,
            toggle_mask,
        })
    }

    pub fn next(&mut self) -> u32 {
        let lsb = self.state & 1u32;
        self.state >>= 1;
        if lsb != 0 {
            self.state ^= self.toggle_mask;
        }
        return self.state;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // TODO
}
