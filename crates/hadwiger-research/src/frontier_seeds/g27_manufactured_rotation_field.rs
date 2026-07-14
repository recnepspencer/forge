#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct K4([Rat; 4]);

impl K4 {
    pub(super) fn zero() -> Self {
        Self([Rat::zero(); 4])
    }

    pub(super) fn one() -> Self {
        Self::rational(1, 1)
    }

    pub(super) fn rational(numerator: i128, denominator: i128) -> Self {
        Self::basis(0, numerator, denominator)
    }

    pub(super) fn sqrt3(numerator: i128, denominator: i128) -> Self {
        Self::basis(1, numerator, denominator)
    }

    pub(super) fn sqrt11(numerator: i128, denominator: i128) -> Self {
        Self::basis(2, numerator, denominator)
    }

    pub(super) fn sqrt33(numerator: i128, denominator: i128) -> Self {
        Self::basis(3, numerator, denominator)
    }

    fn basis(index: usize, numerator: i128, denominator: i128) -> Self {
        let mut values = [Rat::zero(); 4];
        values[index] = Rat::new(numerator, denominator);
        Self(values)
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

    pub(super) fn inverse(self) -> Option<Self> {
        let mut rows = [[Rat::zero(); 5]; 4];
        for basis_index in 0..4 {
            let product = self.mul(Self::basis(basis_index, 1, 1));
            for row in 0..4 {
                rows[row][basis_index] = product.0[row];
            }
        }
        for (row, values) in rows.iter_mut().enumerate() {
            values[4] = if row == 0 { Rat::one() } else { Rat::zero() };
        }
        for column in 0..4 {
            let pivot = (column..4).find(|row| !rows[*row][column].is_zero())?;
            rows.swap(column, pivot);
            let divisor = rows[column][column];
            for value in &mut rows[column] {
                *value = value.div(divisor);
            }
            for row in 0..4 {
                if row != column && !rows[row][column].is_zero() {
                    let factor = rows[row][column];
                    for field in 0..5 {
                        rows[row][field] = rows[row][field].sub(factor.mul(rows[column][field]));
                    }
                }
            }
        }
        Some(Self([rows[0][4], rows[1][4], rows[2][4], rows[3][4]]))
    }

    pub(super) fn to_token(self) -> String {
        token(self.0.iter().zip(["1", "sqrt3", "sqrt11", "sqrt33"]))
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct Q8 {
    base: K4,
    radical: K4,
}

impl Q8 {
    pub(super) fn zero() -> Self {
        Self::base(K4::zero())
    }

    pub(super) fn one() -> Self {
        Self::base(K4::one())
    }

    pub(super) fn base(base: K4) -> Self {
        Self {
            base,
            radical: K4::zero(),
        }
    }

    pub(super) fn w(coefficient: K4) -> Self {
        Self {
            base: K4::zero(),
            radical: coefficient,
        }
    }

    pub(super) fn add(self, other: Self) -> Self {
        Self {
            base: self.base.add(other.base),
            radical: self.radical.add(other.radical),
        }
    }

    pub(super) fn sub(self, other: Self) -> Self {
        self.add(other.scale(-1))
    }

    pub(super) fn scale(self, factor: i128) -> Self {
        Self {
            base: self.base.scale(factor),
            radical: self.radical.scale(factor),
        }
    }

    pub(super) fn mul(self, other: Self, w_squared: K4) -> Self {
        Self {
            base: self
                .base
                .mul(other.base)
                .add(self.radical.mul(other.radical).mul(w_squared)),
            radical: self
                .base
                .mul(other.radical)
                .add(self.radical.mul(other.base)),
        }
    }

    pub(super) fn is_one(self) -> bool {
        self.base == K4::one() && self.radical == K4::zero()
    }

    pub(super) fn to_token(self) -> String {
        let mut parts = Vec::new();
        let base = self.base.to_token();
        if base != "0" {
            parts.push(base);
        }
        let radical = token(
            self.radical
                .0
                .iter()
                .zip(["w", "sqrt3*w", "sqrt11*w", "sqrt33*w"]),
        );
        if radical != "0" {
            parts.push(radical);
        }
        if parts.is_empty() {
            "0".to_string()
        } else {
            parts.join(" + ")
        }
    }
}

fn token<'a>(entries: impl Iterator<Item = (&'a Rat, &'static str)>) -> String {
    let parts = entries
        .filter(|(value, _)| !value.is_zero())
        .map(|(value, name)| format!("{}*{}", value.to_token(), name))
        .collect::<Vec<_>>();
    if parts.is_empty() {
        "0".to_string()
    } else {
        parts.join(" + ")
    }
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Rat {
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

    fn new(mut numerator: i128, mut denominator: i128) -> Self {
        if denominator < 0 {
            numerator = -numerator;
            denominator = -denominator;
        }
        let divisor = gcd(numerator.unsigned_abs(), denominator.unsigned_abs()) as i128;
        Self {
            numerator: numerator / divisor,
            denominator: denominator / divisor,
        }
    }

    fn add(self, other: Self) -> Self {
        Self::new(
            self.numerator * other.denominator + other.numerator * self.denominator,
            self.denominator * other.denominator,
        )
    }

    fn sub(self, other: Self) -> Self {
        self.add(other.scale(-1))
    }

    fn mul(self, other: Self) -> Self {
        Self::new(
            self.numerator * other.numerator,
            self.denominator * other.denominator,
        )
    }

    fn div(self, other: Self) -> Self {
        Self::new(
            self.numerator * other.denominator,
            self.denominator * other.numerator,
        )
    }

    fn scale(self, factor: i128) -> Self {
        Self::new(self.numerator * factor, self.denominator)
    }

    fn is_zero(&self) -> bool {
        self.numerator == 0
    }

    fn to_token(self) -> String {
        if self.denominator == 1 {
            self.numerator.to_string()
        } else {
            format!("{}/{}", self.numerator, self.denominator)
        }
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
