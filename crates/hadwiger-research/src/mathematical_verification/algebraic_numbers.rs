use std::collections::BTreeMap;

use super::algebraic_unit_distance::HadwigerAlgebraicGeometryError;
use super::ExactRational;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct QuadraticFieldElement {
    terms: BTreeMap<i128, ExactRational>,
}

pub type AlgebraicScalar = QuadraticFieldElement;

impl QuadraticFieldElement {
    pub fn rational(value: ExactRational) -> Self {
        Self::from_terms([(0, value)])
    }

    pub fn integer(value: i128) -> Self {
        Self::rational(ExactRational::integer(value))
    }

    pub fn quadratic(
        rational: ExactRational,
        radical_coefficient: ExactRational,
        squarefree_radicand: i128,
    ) -> Result<Self, HadwigerAlgebraicGeometryError> {
        if squarefree_radicand <= 1 || !is_squarefree(squarefree_radicand as u128) {
            return Err(HadwigerAlgebraicGeometryError::NonSquarefreeRadicand {
                radicand: squarefree_radicand,
            });
        }
        Ok(Self::from_terms([
            (0, rational),
            (squarefree_radicand, radical_coefficient),
        ]))
    }

    fn from_terms(terms: impl IntoIterator<Item = (i128, ExactRational)>) -> Self {
        let mut normalized = BTreeMap::new();
        for (radicand, coefficient) in terms {
            if coefficient.is_zero() {
                continue;
            }
            let radicand = if radicand == 1 { 0 } else { radicand };
            let current = normalized
                .remove(&radicand)
                .unwrap_or_else(ExactRational::zero);
            let next = current.add(&coefficient);
            if !next.is_zero() {
                normalized.insert(radicand, next);
            }
        }
        Self { terms: normalized }
    }

    fn add(&self, other: &Self) -> Self {
        Self::from_terms(
            self.terms
                .iter()
                .chain(other.terms.iter())
                .map(|(radicand, coefficient)| (*radicand, coefficient.clone())),
        )
    }

    fn sub(&self, other: &Self) -> Self {
        let negated = other
            .terms
            .iter()
            .map(|(radicand, coefficient)| (*radicand, ExactRational::zero().sub(coefficient)));
        Self::from_terms(
            self.terms
                .iter()
                .map(|(radicand, coefficient)| (*radicand, coefficient.clone()))
                .chain(negated),
        )
    }

    fn square(&self) -> Self {
        self.mul(self)
    }

    fn mul(&self, other: &Self) -> Self {
        let mut terms = Vec::new();
        for (left_radicand, left_coefficient) in &self.terms {
            for (right_radicand, right_coefficient) in &other.terms {
                let (radicand, outside) = multiply_radicands(*left_radicand, *right_radicand);
                terms.push((
                    radicand,
                    left_coefficient
                        .mul(right_coefficient)
                        .mul(&ExactRational::integer(outside)),
                ));
            }
        }
        Self::from_terms(terms)
    }

    pub(super) fn is_one(&self) -> bool {
        self.terms.len() == 1 && self.terms.get(&0) == Some(&ExactRational::integer(1))
    }

    pub(super) fn stable_token(&self) -> String {
        if self.terms.is_empty() {
            "0:0/1".to_string()
        } else {
            self.terms
                .iter()
                .map(|(radicand, coefficient)| format!("{radicand}:{}", coefficient.stable_token()))
                .collect::<Vec<_>>()
                .join(",")
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct AlgebraicPoint2 {
    x: AlgebraicScalar,
    y: AlgebraicScalar,
}

impl AlgebraicPoint2 {
    pub fn rational_integer(x: i128, y: i128) -> Self {
        Self {
            x: AlgebraicScalar::integer(x),
            y: AlgebraicScalar::integer(y),
        }
    }

    pub fn new(x: AlgebraicScalar, y: AlgebraicScalar) -> Self {
        Self { x, y }
    }

    pub(super) fn squared_distance(&self, other: &Self) -> AlgebraicScalar {
        self.x
            .sub(&other.x)
            .square()
            .add(&self.y.sub(&other.y).square())
    }

    pub(super) fn stable_token(&self) -> String {
        format!("{},{}", self.x.stable_token(), self.y.stable_token())
    }
}

pub(super) fn parse_algebraic_scalar(
    value: &str,
) -> Result<AlgebraicScalar, HadwigerAlgebraicGeometryError> {
    if value.contains(':') {
        return parse_canonical_terms(value);
    }
    let (rational, radical) = value.split_once('+').ok_or(empty("algebraic_scalar"))?;
    let (coefficient, radicand) = radical.split_once("*sqrt(").ok_or(empty("radical_term"))?;
    let radicand = radicand
        .strip_suffix(')')
        .ok_or(empty("radicand"))?
        .parse::<i128>()
        .map_err(|_| empty("radicand"))?;
    let rational = parse_rational(rational)?;
    let coefficient = parse_rational(coefficient)?;
    if radicand == 0 && coefficient.is_zero() {
        Ok(AlgebraicScalar::rational(rational))
    } else {
        AlgebraicScalar::quadratic(rational, coefficient, radicand)
    }
}

fn parse_canonical_terms(value: &str) -> Result<AlgebraicScalar, HadwigerAlgebraicGeometryError> {
    let mut terms = Vec::new();
    for term in value.split(',') {
        let (radicand, coefficient) = term
            .split_once(':')
            .ok_or_else(|| unsupported_scalar(value))?;
        let radicand = radicand
            .parse::<i128>()
            .map_err(|_| unsupported_scalar(value))?;
        if radicand < 0 || radicand == 1 {
            return Err(unsupported_scalar(value));
        }
        if radicand > 1 && !is_squarefree(radicand as u128) {
            return Err(HadwigerAlgebraicGeometryError::NonSquarefreeRadicand { radicand });
        }
        terms.push((radicand, parse_rational(coefficient)?));
    }
    Ok(AlgebraicScalar::from_terms(terms))
}

fn unsupported_scalar(value: &str) -> HadwigerAlgebraicGeometryError {
    HadwigerAlgebraicGeometryError::UnsupportedAlgebraicScalar {
        value: value.to_string(),
    }
}

fn parse_rational(value: &str) -> Result<ExactRational, HadwigerAlgebraicGeometryError> {
    let (numerator, denominator) = value.split_once('/').ok_or(empty("rational"))?;
    let numerator = numerator.parse::<i128>().map_err(|_| empty("numerator"))?;
    let denominator = denominator
        .parse::<i128>()
        .map_err(|_| empty("denominator"))?;
    ExactRational::fraction(numerator, denominator)
        .map_err(|_| HadwigerAlgebraicGeometryError::ZeroDenominator)
}

fn empty(field: &'static str) -> HadwigerAlgebraicGeometryError {
    HadwigerAlgebraicGeometryError::EmptyField { field }
}

fn is_squarefree(value: u128) -> bool {
    let mut divisor = 2;
    while divisor * divisor <= value {
        if value % (divisor * divisor) == 0 {
            return false;
        }
        divisor += 1;
    }
    true
}

fn multiply_radicands(left: i128, right: i128) -> (i128, i128) {
    match (left, right) {
        (0, radicand) | (radicand, 0) => (radicand, 1),
        _ => {
            let (outside, squarefree) = squarefree_factor((left * right) as u128);
            let radicand = if squarefree == 1 {
                0
            } else {
                squarefree as i128
            };
            (radicand, outside as i128)
        }
    }
}

fn squarefree_factor(mut value: u128) -> (u128, u128) {
    let mut outside = 1;
    let mut divisor = 2;
    while divisor * divisor <= value {
        while value % (divisor * divisor) == 0 {
            value /= divisor * divisor;
            outside *= divisor;
        }
        divisor += 1;
    }
    (outside, value)
}
