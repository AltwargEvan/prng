use std::{
    collections::VecDeque,
    io::{Error, ErrorKind},
};

pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Xor,
}

pub struct Config {
    modulus: u64,
    j: usize,
    k: usize,
    seed: Box<[u64]>, // seed array length should be equal to k
    operator: Operator,
}

impl Config {
    fn validate(&self) -> Result<(), Error> {
        if self.j >= self.k {
            return Err(Error::new(ErrorKind::InvalidInput, "j must be less than k"));
        } else if self.seed.len() != self.k {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "seed lenth should equal k",
            ));
        }
        Ok(())
    }
}

pub struct LaggedFibonacciGenerator {
    config: Config,
    data: VecDeque<u64>,
}

impl LaggedFibonacciGenerator {
    pub fn new(config: Config) -> Result<LaggedFibonacciGenerator, Error> {
        match config.validate() {
            Err(err) => return Err(err),
            _ => (),
        }

        // populate data property with seed data
        // use veqdeque. this is optimized for push_back and pop_front operations
        let mut data = VecDeque::with_capacity(config.k as usize);
        for val in config.seed.iter() {
            data.push_back(*val);
        }

        Ok(LaggedFibonacciGenerator { config, data })
    }

    pub fn next(&mut self) -> u64 {
        let a = self.data[self.config.j - 1];
        let b = self.data[self.config.k - 1];
        let val = match self.config.operator {
            Operator::Add => a + b,
            Operator::Subtract => a - b,
            Operator::Multiply => a * b,
            Operator::Xor => a ^ b,
        } % self.config.modulus;

        // update state
        self.data.pop_front();
        self.data.push_back(val);
        val
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(config: Config, expected: &[u64]) {
        let mut generator = LaggedFibonacciGenerator::new(config).unwrap();
        for val in expected {
            assert_eq!(
                generator.next(),
                *val,
                "generated value should match expected value"
            );
        }
    }

    #[test]
    fn one() {
        // test data from https://asecuritysite.com/encryption/fab
        const EXPECTED: [u64; 11] = [5, 6, 4, 3, 6, 1, 7, 1, 4, 0, 1];
        test(
            Config {
                modulus: 10,
                j: 3,
                k: 7,
                seed: Box::new([6, 4, 2, 1, 8, 9, 3]),
                operator: Operator::Add,
            },
            &EXPECTED,
        );
    }
}
