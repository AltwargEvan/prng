#![feature(bigint_helper_methods)]

use std::{
    convert::TryFrom,
    io::{Error, ErrorKind},
};

pub struct Config {
    modulus: u64,
    multiplier: u64,
    increment: u64,
    seed: u32,
    output_bits: Option<(u8, u8)>,
}

impl Config {
    fn validate(&self) -> Result<(), Error> {
        if self.multiplier >= self.modulus {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "multiplier must be less than modulus",
            ));
        } else if self.increment >= self.modulus {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "increment must be less than modulus",
            ));
        } else if self.seed as u64 >= self.modulus {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "seed must be less than modulus",
            ));
        }

        match self.output_bits {
            Some(bits) => {
                if bits.0 >= bits.1 {
                    return Err(Error::new(
                        ErrorKind::InvalidInput,
                        "output bit 0 must be less than output bit 1",
                    ));
                } else if bits.0 > 63 {
                    return Err(Error::new(
                        ErrorKind::InvalidInput,
                        "output bits must be than than or equal to 63",
                    ));
                }
            }
            None => (),
        }

        Ok(())
    }
}

pub enum LCGConfig {
    Custom(Config),
    Zx81(u32),
    Ranqd1(u32),
    Borland(u32),
    Glibc(u32),
    MinstdRand0(u32),
    MinstdRand(u32),
    Posix(u32),
    Randu(u32),
}

impl TryFrom<LCGConfig> for Config {
    type Error = Error;

    fn try_from(item: LCGConfig) -> Result<Self, Self::Error> {
        let config: Config = match item {
            LCGConfig::Custom(data) => data,
            LCGConfig::Zx81(seed) => Config {
                modulus: 65537,
                multiplier: 75,
                increment: 74,
                seed,
                output_bits: None,
            },
            LCGConfig::Ranqd1(seed) => Config {
                modulus: 4294967296,
                multiplier: 1664525,
                increment: 1013904223,
                seed,
                output_bits: None,
            },
            LCGConfig::Borland(seed) => Config {
                modulus: 2147483648,
                multiplier: 22695477,
                increment: 1,
                output_bits: Some((16, 30)),
                seed,
            },
            LCGConfig::Glibc(seed) => Config {
                modulus: 2147483648,
                multiplier: 1103515245,
                increment: 12345,
                output_bits: Some((0, 30)),
                seed,
            },
            LCGConfig::MinstdRand0(seed) => Config {
                modulus: 2147483647,
                multiplier: 16807,
                increment: 0,
                output_bits: None,
                seed,
            },
            LCGConfig::MinstdRand(seed) => Config {
                modulus: 2147483647,
                multiplier: 48271,
                increment: 0,
                output_bits: None,
                seed,
            },
            LCGConfig::Posix(seed) => Config {
                modulus: 281474976710656,
                multiplier: 25214903917,
                increment: 11,
                output_bits: Some((16, 47)),
                seed,
            },
            LCGConfig::Randu(seed) => Config {
                modulus: 2147483648,
                multiplier: 65539,
                increment: 0,
                output_bits: None,
                seed,
            },
        };

        match config.validate() {
            Err(err) => Err(err),
            Ok(()) => Ok(config),
        }
    }
}

pub struct LinearCongruentialGenerator(Config);

impl LinearCongruentialGenerator {
    pub fn new(lcg_config: LCGConfig) -> Result<LinearCongruentialGenerator, Error> {
        match Config::try_from(lcg_config) {
            Ok(config) => Ok(LinearCongruentialGenerator(config)),
            Err(err) => Err(err),
        }
    }

    pub fn next(&mut self) -> u32 {
        // X_{n+1} =aX_{n}+c mod m
        let product = self.0.multiplier as u128 * self.0.seed as u128 + self.0.increment as u128;
        let next_seed = (product % self.0.modulus as u128) as u64;

        assert!(
            self.0.seed <= std::u32::MAX.into(),
            "new seed should fit in a u32"
        );
        self.0.seed = next_seed as u32;

        match self.0.output_bits {
            Some((start, end)) => {
                // Not sure if this range is supposed to be inclusive or exclusive
                // wikipedia is unclear about this or I can't read
                println!("premask {}", next_seed);
                let mask = u64::MAX >> (64 - end);
                let clipped_end = next_seed & mask;
                (clipped_end >> start) as u32
            }
            None => next_seed as u32,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(lcg_config: LCGConfig, expected: &[u32]) {
        let mut generator = LinearCongruentialGenerator::new(lcg_config).unwrap();
        for val in expected {
            assert_eq!(
                generator.next(),
                *val,
                "generated value should match expected value"
            );
        }
    }

    #[test]
    fn minstdrand0() {
        const EXPECTED: [u32; 5] = [16807, 282475249, 1622650073, 984943658, 1144108930];
        test(LCGConfig::MinstdRand0(1), &EXPECTED);
    }
    #[test]

    fn randu() {
        // https://oeis.org/A096555
        const EXPECTED: [u32; 20] = [
            65539, 393225, 1769499, 7077969, 26542323, 95552217, 334432395, 1146624417, 1722371299,
            14608041, 1766175739, 1875647473, 1800754131, 366148473, 1022489195, 692115265,
            1392739779, 2127401289, 229749723, 1559239569,
        ];
        test(LCGConfig::Randu(1), &EXPECTED);
    }

    #[test]
    fn borland() {
        const EXPECTED: [u32; 5] = [346, 130, 10982, 1090, 11656];
        test(LCGConfig::Borland(1), &EXPECTED);
    }
}
