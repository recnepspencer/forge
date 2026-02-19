//! Tri-valued sign type and certified predicate results.
//!
//! # Architecture
//!
//! `TriSign` represents the three possible outcomes of a geometric predicate:
//! Negative, Zero, or Positive. Unlike boolean predicates, Zero is meaningful —
//! it represents genuine geometric coincidence (Doctrine D0).
//!
//! `CertifiedTriSign` is a newtype that can ONLY be constructed inside this crate
//! (via `pub(crate)`). This enforces Doctrine D3 at compile time: topology
//! functions accept `CertifiedTriSign`, making it impossible to pass a raw
//! float comparison to a topology mutation.

/// Three-valued sign result from a geometric predicate.
///
/// Unlike boolean classification:
/// - `Zero` represents genuine geometric coincidence (e.g., coplanar points)
/// - `Zero` is NOT a degeneracy to perturb away — it's meaningful geometry
///
/// # Examples
/// ```
/// use forge_math::sign::TriSign;
///
/// let sign = TriSign::Pos;
/// assert!(sign.is_positive());
/// assert!(!sign.is_zero());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum TriSign {
    /// Strictly negative (e.g., point is below a plane)
    Neg,
    /// Exactly zero (e.g., point lies on a plane — genuine coincidence)
    Zero,
    /// Strictly positive (e.g., point is above a plane)
    Pos,
}

impl TriSign {
    /// Returns `true` if the sign is strictly negative.
    pub fn is_negative(self) -> bool {
        matches!(self, TriSign::Neg)
    }

    /// Returns `true` if the sign is exactly zero.
    pub fn is_zero(self) -> bool {
        matches!(self, TriSign::Zero)
    }

    /// Returns `true` if the sign is strictly positive.
    pub fn is_positive(self) -> bool {
        matches!(self, TriSign::Pos)
    }

    /// Returns the negation of this sign.
    pub fn negate(self) -> Self {
        match self {
            TriSign::Neg => TriSign::Pos,
            TriSign::Zero => TriSign::Zero,
            TriSign::Pos => TriSign::Neg,
        }
    }

    /// Multiplies two signs (sign of the product).
    pub fn multiply(self, other: TriSign) -> TriSign {
        match (self, other) {
            (TriSign::Zero, _) | (_, TriSign::Zero) => TriSign::Zero,
            (TriSign::Pos, s) | (s, TriSign::Pos) => s,
            (TriSign::Neg, TriSign::Neg) => TriSign::Pos,
        }
    }
}

impl std::fmt::Display for TriSign {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TriSign::Neg => write!(f, "-"),
            TriSign::Zero => write!(f, "0"),
            TriSign::Pos => write!(f, "+"),
        }
    }
}

/// A certified predicate result that can only be produced by verified predicates
/// inside `forge-math`.
///
/// # Compile-Time Safety (Doctrine D3)
///
/// `CertifiedTriSign` enforces the topology-geometry firewall:
/// - **Construction**: Only possible inside `forge-math` via `pub(crate) fn new()`
/// - **Reading**: Any crate can read the sign via `.sign()`
/// - **Effect**: Topology functions in `forge-topo` accept `CertifiedTriSign`,
///   making it a compile error to pass a raw float comparison
///
/// This ensures every topological decision is backed by a mathematically
/// certified predicate evaluation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CertifiedTriSign(TriSign);

impl CertifiedTriSign {
    /// Construct a new certified sign result.
    ///
    /// This is `pub(crate)` — only callable from within `forge-math` predicates.
    /// External crates cannot construct a `CertifiedTriSign`, enforcing that
    /// all topology decisions flow through certified predicates.
    pub(crate) fn new(sign: TriSign) -> Self {
        Self(sign)
    }

    /// Read the certified sign value.
    pub fn sign(&self) -> TriSign {
        self.0
    }

    /// Returns `true` if the certified sign is negative.
    pub fn is_negative(&self) -> bool {
        self.0.is_negative()
    }

    /// Returns `true` if the certified sign is zero (genuine coincidence).
    pub fn is_zero(&self) -> bool {
        self.0.is_zero()
    }

    /// Returns `true` if the certified sign is positive.
    pub fn is_positive(&self) -> bool {
        self.0.is_positive()
    }
}

impl std::fmt::Display for CertifiedTriSign {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Certified({})", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tri_sign_negation() {
        assert_eq!(TriSign::Neg.negate(), TriSign::Pos);
        assert_eq!(TriSign::Pos.negate(), TriSign::Neg);
        assert_eq!(TriSign::Zero.negate(), TriSign::Zero);
    }

    #[test]
    fn tri_sign_multiplication() {
        assert_eq!(TriSign::Pos.multiply(TriSign::Pos), TriSign::Pos);
        assert_eq!(TriSign::Neg.multiply(TriSign::Neg), TriSign::Pos);
        assert_eq!(TriSign::Pos.multiply(TriSign::Neg), TriSign::Neg);
        assert_eq!(TriSign::Zero.multiply(TriSign::Pos), TriSign::Zero);
        assert_eq!(TriSign::Neg.multiply(TriSign::Zero), TriSign::Zero);
    }

    #[test]
    fn certified_sign_readable() {
        let cert = CertifiedTriSign::new(TriSign::Pos);
        assert_eq!(cert.sign(), TriSign::Pos);
        assert!(cert.is_positive());
        assert!(!cert.is_zero());
        assert!(!cert.is_negative());
    }

    #[test]
    fn certified_sign_display() {
        let cert = CertifiedTriSign::new(TriSign::Zero);
        assert_eq!(format!("{}", cert), "Certified(0)");
    }
}
