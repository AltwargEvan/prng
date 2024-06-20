pub struct MiddleSquareGenerator {
    seed: u64,
    digits: usize,
}

impl MiddleSquareGenerator {
    pub fn new(seed: u64, digits: usize) -> MiddleSquareGenerator {
        MiddleSquareGenerator { seed, digits }
    }

    pub fn next(&mut self) -> u64 {
        let square = self.seed * self.seed;
        let digits = square
            .to_string()
            .chars()
            .map(|d| d.to_digit(10).unwrap())
            .collect::<Vec<_>>();

        let end = digits.len() - self.digits.div_ceil(2);
        let start = end.checked_sub(self.digits).unwrap_or_else(|| 0); // pad 0s

        // exit when start == end. We dont want to slice the array. just assume padded 0s
        if start == end {
            return 0;
        }

        self.seed = digits[start..end]
            .iter()
            .map(|d| d.to_string())
            .collect::<String>()
            .parse::<u64>()
            .unwrap();

        self.seed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wikipedia_sequence_a() {
        // seq from https://en.wikipedia.org/wiki/Middle-square_method#/media/File:Middle_square_method_2_digits.svg
        let mut generator = MiddleSquareGenerator::new(07, 2);
        assert_eq!(generator.next(), 4_u64,);
        assert_eq!(generator.next(), 1_u64);
        assert_eq!(generator.next(), 0_u64);
    }

    #[test]
    fn wikipedia_sequence_b() {
        // seq from https://en.wikipedia.org/wiki/Middle-square_method#/media/File:Middle_square_method_2_digits.svg
        let mut generator = MiddleSquareGenerator::new(42, 2);
        assert_eq!(generator.next(), 76_u64,);
        assert_eq!(generator.next(), 77_u64);
        assert_eq!(generator.next(), 92_u64);
        assert_eq!(generator.next(), 46_u64);
        assert_eq!(generator.next(), 11_u64);
        assert_eq!(generator.next(), 12_u64);
        assert_eq!(generator.next(), 14_u64);
        assert_eq!(generator.next(), 19_u64);
        assert_eq!(generator.next(), 36_u64);
        assert_eq!(generator.next(), 29_u64);
        assert_eq!(generator.next(), 84_u64);
        assert_eq!(generator.next(), 5_u64);
        assert_eq!(generator.next(), 2_u64);
        assert_eq!(generator.next(), 0_u64);
    }
    #[test]

    fn wikipedia_sequence_c() {
        // seq from https://en.wikipedia.org/wiki/Middle-square_method#/media/File:Middle_square_method_2_digits.svg
        let mut generator = MiddleSquareGenerator::new(81, 2);
        assert_eq!(generator.next(), 56_u64);
        assert_eq!(generator.next(), 13_u64);
        assert_eq!(generator.next(), 16_u64);
        assert_eq!(generator.next(), 25_u64);
        assert_eq!(generator.next(), 62_u64);
        assert_eq!(generator.next(), 84_u64);
        assert_eq!(generator.next(), 5_u64);
        assert_eq!(generator.next(), 2_u64);
        assert_eq!(generator.next(), 0_u64);
    }

    #[test]

    fn wikipedia_sequence_d() {
        // seq from https://en.wikipedia.org/wiki/Middle-square_method#/media/File:Middle_square_method_2_digits.svg
        let mut generator = MiddleSquareGenerator::new(31, 2);
        assert_eq!(generator.next(), 96_u64);
        assert_eq!(generator.next(), 21_u64);
        assert_eq!(generator.next(), 44_u64);
        assert_eq!(generator.next(), 93_u64);
        assert_eq!(generator.next(), 64_u64);
        assert_eq!(generator.next(), 9_u64);
        assert_eq!(generator.next(), 8_u64);
        assert_eq!(generator.next(), 6_u64);
        assert_eq!(generator.next(), 3_u64);
        assert_eq!(generator.next(), 0_u64);
    }
    #[test]

    fn wikipedia_sequence_e() {
        // seq from https://en.wikipedia.org/wiki/Middle-square_method#/media/File:Middle_square_method_2_digits.svg
        let mut generator = MiddleSquareGenerator::new(94, 2);
        assert_eq!(generator.next(), 83_u64);
        assert_eq!(generator.next(), 88_u64);
        assert_eq!(generator.next(), 74_u64);
        assert_eq!(generator.next(), 47_u64);
        assert_eq!(generator.next(), 20_u64);
        assert_eq!(generator.next(), 40_u64);
        assert_eq!(generator.next(), 60_u64);
        assert_eq!(generator.next(), 60_u64);
    }
}
