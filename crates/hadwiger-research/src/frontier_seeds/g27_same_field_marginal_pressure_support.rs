use std::cmp::Ordering;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct NormalizedScore {
    numerator: i128,
    denominator: i128,
}

impl Ord for NormalizedScore {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.numerator * other.denominator).cmp(&(other.numerator * self.denominator))
    }
}

impl PartialOrd for NormalizedScore {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl NormalizedScore {
    pub(super) fn zero() -> Self {
        Self {
            numerator: 0,
            denominator: 1,
        }
    }

    pub(super) fn new(numerator: i128, denominator: i128) -> Self {
        let divisor = gcd(numerator.unsigned_abs(), denominator as u128) as i128;
        Self {
            numerator: numerator / divisor,
            denominator: denominator / divisor,
        }
    }

    pub(super) fn add(self, other: Self) -> Self {
        Self::new(
            self.numerator * other.denominator + other.numerator * self.denominator,
            self.denominator * other.denominator,
        )
    }

    pub(super) fn div(self, other: Self) -> Self {
        if other.numerator == 0 {
            return Self::zero();
        }
        Self::new(
            self.numerator * other.denominator,
            self.denominator * other.numerator,
        )
    }

    pub(super) fn cmp_integer(self, value: i128) -> Ordering {
        self.numerator.cmp(&(value * self.denominator))
    }

    pub(super) fn cmp_fraction(self, numerator: i128, denominator: i128) -> Ordering {
        (self.numerator * denominator).cmp(&(numerator * self.denominator))
    }

    pub(super) fn stable_token(self) -> String {
        format!("{}/{}", self.numerator, self.denominator)
    }
}

fn gcd(mut left: u128, mut right: u128) -> u128 {
    while right != 0 {
        let next = left % right;
        left = right;
        right = next;
    }
    left.max(1)
}
