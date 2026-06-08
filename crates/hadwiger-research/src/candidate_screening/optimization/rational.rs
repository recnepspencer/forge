use std::cmp::Ordering;

use crate::domain_artifacts::HadwigerArtifactShapeError;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ScreeningRational {
    numerator: i128,
    denominator: i128,
}

impl PartialOrd for ScreeningRational {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ScreeningRational {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.numerator * other.denominator).cmp(&(other.numerator * self.denominator))
    }
}

impl ScreeningRational {
    pub fn integer(value: i128) -> Self {
        Self {
            numerator: value,
            denominator: 1,
        }
    }

    pub fn fraction(
        numerator: i128,
        denominator: i128,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        if denominator == 0 {
            return Err(HadwigerArtifactShapeError::EmptyField {
                field: "rational_denominator",
            });
        }
        let mut numerator = numerator;
        let mut denominator = denominator;
        if denominator < 0 {
            numerator = -numerator;
            denominator = -denominator;
        }
        let divisor = gcd(numerator.unsigned_abs(), denominator as u128) as i128;
        Ok(Self {
            numerator: numerator / divisor,
            denominator: denominator / divisor,
        })
    }

    pub(crate) fn approximate_from_f64(
        value: f64,
        max_denominator: i128,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        if !value.is_finite() || max_denominator <= 0 {
            return Err(HadwigerArtifactShapeError::EmptyField {
                field: "finite_rational_approximation",
            });
        }
        let sign = if value < 0.0 { -1 } else { 1 };
        let scaled = (value.abs() * max_denominator as f64).round() as i128;
        Self::fraction(sign * scaled, max_denominator)
    }

    pub(crate) fn add(&self, other: &Self) -> Self {
        Self::fraction(
            self.numerator * other.denominator + other.numerator * self.denominator,
            self.denominator * other.denominator,
        )
        .expect("rational addition keeps non-zero denominator")
    }

    pub(crate) fn sub(&self, other: &Self) -> Self {
        Self::fraction(
            self.numerator * other.denominator - other.numerator * self.denominator,
            self.denominator * other.denominator,
        )
        .expect("rational subtraction keeps non-zero denominator")
    }

    pub(crate) fn mul(&self, other: &Self) -> Self {
        Self::fraction(
            self.numerator * other.numerator,
            self.denominator * other.denominator,
        )
        .expect("rational multiplication keeps non-zero denominator")
    }

    pub(crate) fn is_positive(&self) -> bool {
        self.numerator > 0
    }

    pub(crate) fn is_negative(&self) -> bool {
        self.numerator < 0
    }

    pub(crate) fn is_zero(&self) -> bool {
        self.numerator == 0
    }

    pub(crate) fn cmp_integer(&self, value: i128) -> Ordering {
        self.numerator.cmp(&(value * self.denominator))
    }

    pub fn stable_token(&self) -> String {
        format!("{}/{}", self.numerator, self.denominator)
    }
}

fn gcd(mut left: u128, mut right: u128) -> u128 {
    while right != 0 {
        let remainder = left % right;
        left = right;
        right = remainder;
    }
    left.max(1)
}
