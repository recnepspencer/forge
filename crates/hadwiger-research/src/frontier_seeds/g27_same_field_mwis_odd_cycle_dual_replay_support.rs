use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{One, Zero};

use super::g27_same_field_mwis_odd_cycle_row_replay_support::NodeRecord;

pub(super) const MAX_DENOMINATOR: i128 = 1_000_000;
pub(super) const POSITIVE_EPSILON: f64 = 1.0e-9;

#[derive(Clone)]
pub(super) struct ExplicitRow {
    pub(super) support: Vec<usize>,
    pub(super) rhs: i128,
}

pub(super) struct CertifiedRow {
    pub(super) index: usize,
    pub(super) multiplier: Rational,
}

pub(super) struct ReplayResult {
    pub(super) coverage_ok: bool,
    pub(super) objective_ceil: i128,
    pub(super) min_slack_floor: i128,
}

pub(super) fn replay_certificate(
    weights: &[i128],
    record: &NodeRecord,
    rows: &[ExplicitRow],
    certificate: &[CertifiedRow],
) -> ReplayResult {
    replay_certificate_for_candidates(weights, record.candidates(), rows, certificate)
}

pub(super) fn replay_certificate_for_candidates(
    weights: &[i128],
    candidates: &[usize],
    rows: &[ExplicitRow],
    certificate: &[CertifiedRow],
) -> ReplayResult {
    let mut coverage = vec![Rational::zero(); candidates.len()];
    let mut objective = Rational::zero();
    for certified in certificate {
        let row = &rows[certified.index];
        objective.add_assign(&certified.multiplier.mul_i128(row.rhs));
        for vertex in &row.support {
            coverage[*vertex].add_assign(&certified.multiplier);
        }
    }
    let mut coverage_ok = true;
    let mut min_slack_floor = i128::MAX;
    for (local, covered) in coverage.iter().enumerate() {
        let weight = weights[candidates[local]];
        if !covered.ge_i128(weight) {
            coverage_ok = false;
        }
        min_slack_floor = min_slack_floor.min(covered.sub_i128(weight).floor_i128());
    }
    ReplayResult {
        coverage_ok,
        objective_ceil: objective.ceil_i128(),
        min_slack_floor,
    }
}

#[derive(Clone)]
pub(super) struct Rational {
    numerator: BigInt,
    denominator: BigInt,
}

impl Rational {
    fn zero() -> Self {
        Self {
            numerator: BigInt::zero(),
            denominator: BigInt::one(),
        }
    }

    pub(super) fn nearest_from_f64(value: f64) -> Self {
        let (num, den) = approximate_fraction(value.max(0.0), MAX_DENOMINATOR);
        Self::from_i128s(num, den)
    }

    pub(super) fn ceiling_from_f64(value: f64) -> Self {
        if value <= POSITIVE_EPSILON {
            return Self::zero();
        }
        let denominator = MAX_DENOMINATOR;
        let numerator = (value * denominator as f64).ceil() as i128;
        Self::from_i128s(numerator, denominator)
    }

    fn from_i128s(numerator: i128, denominator: i128) -> Self {
        let gcd = numerator.gcd(&denominator).abs();
        Self {
            numerator: BigInt::from(numerator / gcd),
            denominator: BigInt::from(denominator / gcd),
        }
    }

    fn add_assign(&mut self, other: &Self) {
        self.numerator =
            &self.numerator * &other.denominator + &other.numerator * &self.denominator;
        self.denominator *= &other.denominator;
        self.reduce();
    }

    fn mul_i128(&self, factor: i128) -> Self {
        let mut result = Self {
            numerator: &self.numerator * BigInt::from(factor),
            denominator: self.denominator.clone(),
        };
        result.reduce();
        result
    }

    fn sub_i128(&self, value: i128) -> Self {
        let mut result = Self {
            numerator: &self.numerator - BigInt::from(value) * &self.denominator,
            denominator: self.denominator.clone(),
        };
        result.reduce();
        result
    }

    fn ge_i128(&self, value: i128) -> bool {
        &self.numerator >= &(BigInt::from(value) * &self.denominator)
    }

    fn ceil_i128(&self) -> i128 {
        let (q, r) = self.numerator.div_rem(&self.denominator);
        let needs_increment = !r.is_zero();
        bigint_to_i128(
            q + if needs_increment {
                BigInt::one()
            } else {
                BigInt::zero()
            },
        )
    }

    fn floor_i128(&self) -> i128 {
        let (q, r) = self.numerator.div_rem(&self.denominator);
        if self.numerator.sign() == num_bigint::Sign::Minus && !r.is_zero() {
            bigint_to_i128(q - BigInt::one())
        } else {
            bigint_to_i128(q)
        }
    }

    pub(super) fn den_i128(&self) -> i128 {
        bigint_to_i128(self.denominator.clone())
    }

    pub(super) fn num_i128(&self) -> i128 {
        bigint_to_i128(self.numerator.clone())
    }

    fn reduce(&mut self) {
        let gcd = self.numerator.gcd(&self.denominator);
        if gcd > BigInt::one() {
            self.numerator /= &gcd;
            self.denominator /= gcd;
        }
    }
}

fn approximate_fraction(value: f64, max_denominator: i128) -> (i128, i128) {
    if value <= POSITIVE_EPSILON {
        return (0, 1);
    }
    let mut lower = (0_i128, 1_i128);
    let mut upper = (1_i128, 0_i128);
    loop {
        let mediant = (lower.0 + upper.0, lower.1 + upper.1);
        if mediant.1 > max_denominator {
            break;
        }
        let mediant_value = mediant.0 as f64 / mediant.1 as f64;
        if mediant_value < value {
            lower = mediant;
        } else {
            upper = mediant;
        }
        if (mediant_value - value).abs() <= 1.0e-12 {
            return mediant;
        }
    }
    let lower_error = (lower.0 as f64 / lower.1 as f64 - value).abs();
    let upper_error = if upper.1 == 0 {
        f64::INFINITY
    } else {
        (upper.0 as f64 / upper.1 as f64 - value).abs()
    };
    if lower_error <= upper_error {
        lower
    } else {
        upper
    }
}

fn bigint_to_i128(value: BigInt) -> i128 {
    value
        .to_string()
        .parse::<i128>()
        .expect("certificate integer fits i128")
}
