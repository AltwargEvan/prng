#![feature(bigint_helper_methods)]

use std::convert::From;

pub struct MINSTDConfig {
    seed: u32,
}

pub struct ZX81Config {
    seed: u32,
}
pub struct RANDUConfig {
    seed: u32,
}

pub struct BaseConfig {
    modulus: u64,    // prime number or power of a prime number
    multiplier: u64, // element of high multiplicative order modulo modulus
    seed: u32,       // coprime to modulus
}

pub enum Config {
    MINSTDConfig(MINSTDConfig),
    ZX81Config(ZX81Config),
    RANDUConfig(RANDUConfig),
    CustomConfig(BaseConfig),
}

impl From<Config> for BaseConfig {
    fn from(item: Config) -> Self {
        match item {
            Config::MINSTDConfig(data) => BaseConfig {
                multiplier: 16807,
                modulus: 2147483647,
                seed: data.seed,
            },
            Config::ZX81Config(data) => BaseConfig {
                multiplier: 75,
                modulus: 65537,
                seed: data.seed,
            },
            Config::RANDUConfig(data) => BaseConfig {
                multiplier: 65539,
                modulus: 2147483648,
                seed: data.seed,
            },
            Config::CustomConfig(data) => data,
        }
    }
}

pub struct LehmerGenerator {
    multiplier: u64,
    modulus: u64,
    state: u64,
}

impl LehmerGenerator {
    pub fn new(config: Config) -> LehmerGenerator {
        let data = BaseConfig::from(config);
        LehmerGenerator {
            multiplier: data.multiplier as u64,
            modulus: data.modulus as u64,
            state: data.seed as u64,
        }
    }

    pub fn next(&mut self) -> u32 {
        self.state = self.multiplier * self.state % self.modulus;
        assert!(
            self.state <= std::u32::MAX.into(),
            "new state should fit in a u32"
        );
        self.state as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn minstd() {
        const EXPECTED: [u32; 5] = [16807, 282475249, 1622650073, 984943658, 1144108930];
        let mut generator = LehmerGenerator::new(Config::MINSTDConfig(MINSTDConfig { seed: 1 }));
        for ele in EXPECTED {
            assert_eq!(generator.next(), ele);
        }
    }
    #[test]

    fn randu() {
        // https://oeis.org/A096555
        const EXPECTED: [u32; 20] = [
            65539, 393225, 1769499, 7077969, 26542323, 95552217, 334432395, 1146624417, 1722371299,
            14608041, 1766175739, 1875647473, 1800754131, 366148473, 1022489195, 692115265,
            1392739779, 2127401289, 229749723, 1559239569,
        ];
        let mut generator = LehmerGenerator::new(Config::RANDUConfig(RANDUConfig { seed: 1 }));
        for ele in EXPECTED {
            assert_eq!(generator.next(), ele);
        }
    }
    #[test]

    fn zx81() {
        const EXPECTED: [u32; 5] = [75, 5625, 28653, 51791, 17642];
        let mut generator = LehmerGenerator::new(Config::ZX81Config(ZX81Config { seed: 1 }));
        for ele in EXPECTED {
            assert_eq!(generator.next(), ele);
        }
    }
}
