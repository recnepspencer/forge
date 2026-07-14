use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{One, Signed, ToPrimitive, Zero};
use serde::Deserialize;

use super::g27_geometric_fractional::G27GeometricFractionalError;

#[derive(Deserialize)]
pub struct FullTerminalArtifact {
    pub schema: String,
    pub authority: String,
    pub source_binding: SourceBinding,
    pub manifest: Manifest,
    pub summary: Summary,
    pub terminals: Vec<Terminal>,
    pub failure_reasons: Vec<String>,
    pub status: String,
}

#[derive(Deserialize)]
pub struct SourceBinding {
    pub fresh_replay_digest: String,
    pub first_family_digest: String,
    pub graph_digest: String,
    pub root_rows_digest: String,
    pub parent_rows_digest: String,
}

#[derive(Deserialize)]
pub struct Manifest {
    pub expected_terminal_count: usize,
    pub actual_terminal_count: usize,
    pub replaced_triggered_leaf0_terminals: Vec<Replacement>,
    pub duplicate_keys: Vec<String>,
}

#[derive(Deserialize)]
pub struct Replacement {
    pub leaf_index: usize,
    pub terminal_id: String,
    pub replaced_by: usize,
}

#[derive(Deserialize)]
pub struct Summary {
    pub successes: usize,
    pub total: usize,
    pub total_success_rows: usize,
}

#[derive(Deserialize)]
pub struct Terminal {
    pub leaf_index: usize,
    pub terminal_id: String,
    pub expected_bound: f64,
    pub pool_assignment: serde_json::Value,
    pub depth: usize,
    pub mechanism_class: String,
    pub export_required: bool,
    pub residual_pair_assignment: Option<serde_json::Value>,
    pub selected_strategy: String,
    pub certificate: Certificate,
    pub failure_reasons: Vec<String>,
    pub status: String,
}

impl Terminal {
    pub fn expected_bound_floor(&self) -> i64 {
        self.expected_bound.floor() as i64
    }
}

#[derive(Deserialize)]
pub struct Certificate {
    pub strategy: String,
    pub positive_rows: Vec<ProofRow>,
    pub selected: Option<IntegerSelected>,
    pub positive_row_count: Option<usize>,
}

impl Certificate {
    pub fn positive_row_count(&self) -> usize {
        self.positive_row_count
            .or_else(|| {
                self.selected
                    .as_ref()
                    .map(|selected| selected.positive_row_count)
            })
            .unwrap_or(self.positive_rows.len())
    }
}

#[derive(Deserialize)]
pub struct IntegerSelected {
    pub positive_row_count: usize,
}

#[derive(Deserialize)]
pub struct ProofRow {
    pub family: String,
    pub id: String,
    pub rhs: i64,
    pub coefficients: Vec<(usize, i64)>,
    pub multiplier: Multiplier,
}

#[derive(Deserialize)]
pub struct Multiplier {
    pub num: i128,
    pub den: i128,
}

#[derive(Deserialize)]
pub struct FreshReplay {
    pub schema: String,
    pub status: String,
    pub authority_label: String,
    pub leaf_count: usize,
    pub leaves: Vec<FreshLeaf>,
    pub failure_reasons: Vec<String>,
}

#[derive(Deserialize)]
pub struct FreshLeaf {
    pub leaf_index: usize,
    pub exceptional_rule: String,
    pub tier_a_assignment: AssignmentSummary,
    pub terminal_certificates: Vec<FreshTerminal>,
    pub residual_closures: Vec<ResidualClosure>,
}

#[derive(Deserialize)]
pub struct AssignmentSummary {
    pub included: Vec<usize>,
    pub excluded: Vec<usize>,
}

#[derive(Deserialize)]
pub struct FreshTerminal {
    pub pool_assignment: serde_json::Value,
}

#[derive(Deserialize)]
pub struct ResidualClosure {
    pub triggered: bool,
    pub terminal: FreshTerminal,
    pub children: Vec<ResidualChild>,
}

#[derive(Deserialize)]
pub struct ResidualChild {
    pub pool_assignment: serde_json::Value,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Rational {
    num: BigInt,
    den: BigInt,
}

impl Rational {
    pub fn zero() -> Self {
        Self::new(BigInt::zero(), BigInt::one())
    }

    pub fn new(num: BigInt, den: BigInt) -> Self {
        let gcd = num.gcd(&den);
        let mut num = num / &gcd;
        let mut den = den / gcd;
        if den.is_negative() {
            num = -num;
            den = -den;
        }
        Self { num, den }
    }

    pub fn add(&self, other: &Self) -> Self {
        Self::new(
            &self.num * &other.den + &other.num * &self.den,
            &self.den * &other.den,
        )
    }

    pub fn mul_i64(&self, value: i64) -> Self {
        Self::new(&self.num * value, self.den.clone())
    }

    pub fn sub_i128(&self, value: i128) -> Self {
        Self::new(
            &self.num - BigInt::from(value) * &self.den,
            self.den.clone(),
        )
    }

    pub fn min(self, other: Self) -> Self {
        if self <= other {
            self
        } else {
            other
        }
    }

    pub fn is_negative(&self) -> bool {
        self.num.is_negative()
    }

    pub fn gt_i64(&self, value: i64) -> bool {
        &self.num > &(BigInt::from(value) * &self.den)
    }

    pub fn floor_i128(&self) -> i128 {
        (&self.num / &self.den).to_i128().unwrap_or(i128::MAX)
    }

    pub fn ceil_i128(&self) -> i128 {
        let quotient = &self.num / &self.den;
        let remainder = &self.num % &self.den;
        if remainder.is_zero() {
            quotient.to_i128().unwrap_or(i128::MAX)
        } else {
            (quotient + BigInt::one()).to_i128().unwrap_or(i128::MAX)
        }
    }
}

impl PartialOrd for Rational {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some((&self.num * &other.den).cmp(&(&other.num * &self.den)))
    }
}

pub struct TerminalReplay {
    pub rows: usize,
    pub objective: Rational,
    pub min_slack: Rational,
}

pub fn replay_terminal(
    terminal: &Terminal,
    weights: &[i128],
    vertex_count: usize,
) -> Result<TerminalReplay, G27GeometricFractionalError> {
    let mut coverage = vec![Rational::zero(); vertex_count];
    let mut objective = Rational::zero();
    for row in &terminal.certificate.positive_rows {
        if row.multiplier.num <= 0 || row.multiplier.den <= 0 {
            return malformed("w607_full_terminal_multiplier");
        }
        let multiplier = Rational::new(
            BigInt::from(row.multiplier.num),
            BigInt::from(row.multiplier.den),
        );
        objective = objective.add(&multiplier.mul_i64(row.rhs));
        for (vertex, coeff) in &row.coefficients {
            if *vertex == 0 || *vertex > vertex_count {
                return malformed("w607_full_terminal_vertex");
            }
            let contribution = multiplier.mul_i64(*coeff);
            coverage[*vertex - 1] = coverage[*vertex - 1].add(&contribution);
        }
    }
    let mut min_slack = Rational::new(BigInt::from(i128::MAX), BigInt::one());
    for (covered, weight) in coverage.into_iter().zip(weights.iter()) {
        let slack = covered.sub_i128(*weight);
        if slack.is_negative() {
            return malformed("w607_full_terminal_negative_slack");
        }
        min_slack = min_slack.min(slack);
    }
    if terminal.certificate.positive_rows.len() != terminal.certificate.positive_row_count() {
        return malformed("w607_full_terminal_row_count");
    }
    Ok(TerminalReplay {
        rows: terminal.certificate.positive_rows.len(),
        objective,
        min_slack,
    })
}

fn malformed<T>(source: &'static str) -> Result<T, G27GeometricFractionalError> {
    Err(G27GeometricFractionalError::MalformedData { source })
}
