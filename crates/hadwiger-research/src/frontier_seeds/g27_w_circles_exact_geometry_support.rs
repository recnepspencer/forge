use std::collections::BTreeSet;

use super::g27_geometric_fractional::G27GeometricFractionalError;

pub(super) const W_VERTICES: &str =
    include_str!("g27_finite_fractional/W_circles_607_vertices.sage");
pub(super) const W_EDGES: &str = include_str!("g27_finite_fractional/W_circles_607_integers.dat");
pub(super) const EXPECTED_VERTEX_COUNT: usize = 607;
pub(super) const EXPECTED_EDGE_COUNT: usize = 3_390;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct WExactPoint {
    pub(super) x: K4,
    pub(super) y: K4,
}

impl WExactPoint {
    pub(super) fn add(self, other: Self) -> Self {
        Self {
            x: self.x.add(other.x),
            y: self.y.add(other.y),
        }
    }

    pub(super) fn sub(self, other: Self) -> Self {
        Self {
            x: self.x.sub(other.x),
            y: self.y.sub(other.y),
        }
    }

    pub(super) fn scale(self, factor: i128) -> Self {
        Self {
            x: self.x.scale(factor),
            y: self.y.scale(factor),
        }
    }

    pub(super) fn approx(self) -> (f64, f64) {
        (self.x.approx(), self.y.approx())
    }
}

pub(super) fn parse_w_vertices() -> Result<Vec<WExactPoint>, G27GeometricFractionalError> {
    let rationals = rational_tokens(W_VERTICES)?
        .into_iter()
        .map(parse_rat)
        .collect::<Result<Vec<_>, _>>()?;
    if rationals.len() != EXPECTED_VERTEX_COUNT * 8 {
        return Err(malformed("w_circles_607_vertex_shape"));
    }
    Ok(rationals
        .chunks_exact(8)
        .map(|row| WExactPoint {
            x: K4([row[0], row[1], row[2], row[3]]),
            y: K4([row[4], row[5], row[6], row[7]]),
        })
        .collect())
}

pub(super) fn parse_w_retained_edges(
    vertex_count: usize,
) -> Result<BTreeSet<(usize, usize)>, G27GeometricFractionalError> {
    let edge_blob = W_EDGES
        .split_once("Edges = {")
        .and_then(|(_, rest)| rest.split_once("};").map(|(body, _)| body))
        .ok_or(malformed("w_edges_section"))?;
    let mut edges = BTreeSet::new();
    for entry in edge_blob.split('<').skip(1) {
        let pair = entry
            .split_once('>')
            .map(|(pair, _)| pair)
            .ok_or(malformed("w_edge_pair"))?;
        let (left, right) = pair.split_once(',').ok_or(malformed("w_edge_tuple"))?;
        let left = parse_usize(left.trim(), "w_edge_left")?;
        let right = parse_usize(right.trim(), "w_edge_right")?;
        if left == 0 || right == 0 || left > vertex_count || right > vertex_count || left == right {
            return Err(malformed("w_edge_endpoint"));
        }
        edges.insert(if left < right {
            (left, right)
        } else {
            (right, left)
        });
    }
    Ok(edges)
}

pub(super) fn parse_w_integer_weights() -> Result<Vec<i128>, G27GeometricFractionalError> {
    let weight_blob = W_EDGES
        .split_once("w = [")
        .and_then(|(_, rest)| rest.split_once("];").map(|(body, _)| body))
        .ok_or(malformed("w_weights_section"))?;
    weight_blob
        .split(',')
        .filter(|value| !value.trim().is_empty())
        .map(parse_integer_weight)
        .collect()
}

pub(super) fn replay_w_unit_edges(vertices: &[WExactPoint]) -> BTreeSet<(usize, usize)> {
    let mut edges = BTreeSet::new();
    for left in 0..vertices.len() {
        for right in (left + 1)..vertices.len() {
            if squared_distance(vertices[left], vertices[right]).is_one() {
                edges.insert((left + 1, right + 1));
            }
        }
    }
    edges
}

pub(super) fn squared_distance(left: WExactPoint, right: WExactPoint) -> K4 {
    let dx = left.x.sub(right.x);
    let dy = left.y.sub(right.y);
    dx.mul(dx).add(dy.mul(dy))
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct K4(pub(super) [Rat; 4]);

impl K4 {
    pub(super) fn zero() -> Self {
        Self([Rat::zero(); 4])
    }

    pub(super) fn one() -> Self {
        Self([Rat::one(), Rat::zero(), Rat::zero(), Rat::zero()])
    }

    pub(super) fn rational(numerator: i128, denominator: i128) -> Self {
        Self([
            Rat::new(numerator, denominator).expect("literal denominator"),
            Rat::zero(),
            Rat::zero(),
            Rat::zero(),
        ])
    }

    pub(super) fn add(self, other: Self) -> Self {
        let mut values = [Rat::zero(); 4];
        for (index, value) in values.iter_mut().enumerate() {
            *value = self.0[index].add(other.0[index]);
        }
        Self(values)
    }

    pub(super) fn sub(self, other: Self) -> Self {
        self.add(other.scale(-1))
    }

    pub(super) fn scale(self, factor: i128) -> Self {
        let mut values = [Rat::zero(); 4];
        for (index, value) in values.iter_mut().enumerate() {
            *value = self.0[index].scale(factor);
        }
        Self(values)
    }

    pub(super) fn mul(self, other: Self) -> Self {
        let mut values = [Rat::zero(); 4];
        for left in 0..4 {
            for right in 0..4 {
                let (index, factor) = base_product(left, right);
                values[index] = values[index].add(self.0[left].mul(other.0[right]).scale(factor));
            }
        }
        Self(values)
    }

    pub(super) fn is_one(self) -> bool {
        self == Self::one()
    }

    pub(super) fn approx(self) -> f64 {
        self.0[0].approx()
            + self.0[1].approx() * 3f64.sqrt()
            + self.0[2].approx() * 11f64.sqrt()
            + self.0[3].approx() * 33f64.sqrt()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct Rat {
    numerator: i128,
    denominator: i128,
}

impl Rat {
    const fn zero() -> Self {
        Self {
            numerator: 0,
            denominator: 1,
        }
    }

    const fn one() -> Self {
        Self {
            numerator: 1,
            denominator: 1,
        }
    }

    pub(super) fn new(
        numerator: i128,
        denominator: i128,
    ) -> Result<Self, G27GeometricFractionalError> {
        if denominator == 0 {
            return Err(malformed("w_vertex_zero_denominator"));
        }
        let mut numerator = numerator;
        let mut denominator = denominator;
        if denominator < 0 {
            numerator = -numerator;
            denominator = -denominator;
        }
        let divisor = gcd(numerator.unsigned_abs(), denominator.unsigned_abs()) as i128;
        Ok(Self {
            numerator: numerator / divisor,
            denominator: denominator / divisor,
        })
    }

    fn add(self, other: Self) -> Self {
        Self::new(
            self.numerator * other.denominator + other.numerator * self.denominator,
            self.denominator * other.denominator,
        )
        .expect("addition keeps denominator")
    }

    fn mul(self, other: Self) -> Self {
        Self::new(
            self.numerator * other.numerator,
            self.denominator * other.denominator,
        )
        .expect("multiplication keeps denominator")
    }

    fn scale(self, factor: i128) -> Self {
        Self::new(self.numerator * factor, self.denominator).expect("scale keeps denominator")
    }

    fn approx(self) -> f64 {
        self.numerator as f64 / self.denominator as f64
    }
}

fn rational_tokens(source: &str) -> Result<Vec<String>, G27GeometricFractionalError> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    for ch in source.chars() {
        if ch.is_ascii_digit() || ch == '-' || ch == '/' {
            current.push(ch);
        } else if !current.is_empty() {
            tokens.push(std::mem::take(&mut current));
        }
    }
    if !current.is_empty() {
        tokens.push(current);
    }
    Ok(tokens)
}

fn parse_rat(value: String) -> Result<Rat, G27GeometricFractionalError> {
    if let Some((num, den)) = value.split_once('/') {
        Rat::new(
            num.parse().map_err(|_| malformed("w_vertex_rat_num"))?,
            den.parse().map_err(|_| malformed("w_vertex_rat_den"))?,
        )
    } else {
        Rat::new(value.parse().map_err(|_| malformed("w_vertex_integer"))?, 1)
    }
}

fn parse_integer_weight(value: &str) -> Result<i128, G27GeometricFractionalError> {
    let (integer, fraction) = value
        .trim()
        .split_once('.')
        .ok_or(malformed("w_integer_weight"))?;
    if !fraction.chars().all(|ch| ch == '0') {
        return Err(malformed("w_non_integer_weight"));
    }
    integer
        .parse::<i128>()
        .map_err(|_| malformed("w_integer_weight_parse"))
}

fn parse_usize(value: &str, source: &'static str) -> Result<usize, G27GeometricFractionalError> {
    value.parse::<usize>().map_err(|_| malformed(source))
}

fn base_product(left: usize, right: usize) -> (usize, i128) {
    match (left, right) {
        (0, x) | (x, 0) => (x, 1),
        (1, 1) => (0, 3),
        (2, 2) => (0, 11),
        (3, 3) => (0, 33),
        (1, 2) | (2, 1) => (3, 1),
        (1, 3) | (3, 1) => (2, 3),
        (2, 3) | (3, 2) => (1, 11),
        _ => (0, 0),
    }
}

fn malformed(source: &'static str) -> G27GeometricFractionalError {
    G27GeometricFractionalError::MalformedData { source }
}

fn gcd(mut left: u128, mut right: u128) -> u128 {
    while right != 0 {
        let next = left % right;
        left = right;
        right = next;
    }
    left.max(1)
}
