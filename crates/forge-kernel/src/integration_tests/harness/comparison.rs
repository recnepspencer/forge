//! Geometric comparison policy — bounded-error assertions for aerospace-grade tests.
//!
//! DOMAIN: All geometric assertions use `ComparisonPolicy` instead of raw
//! `== `or ad-hoc tolerances. Each policy carries abs/rel tolerance bounds
//! plus the algorithm name for audit trails. The `assert_geo_eq` function
//! reports both the deviation and the policy in failure messages, making
//! every test failure auditable.
//!
//! RULE: Never use `==` on f64 in geometric tests. Use `assert_geo_eq`.

/// Comparison policy for geometric assertions.
///
/// Encodes both absolute and relative tolerance bounds, plus the name of the
/// algorithm that produced the values being compared. Failure messages include
/// the full policy so test failures are self-documenting.
#[derive(Debug, Clone, Copy)]
pub struct ComparisonPolicy {
    /// Maximum absolute deviation permitted.
    pub abs_tol: f64,
    /// Maximum relative deviation permitted (relative to `max(|a|, |b|)`).
    pub rel_tol: f64,
    /// Algorithm name for audit (e.g., "divergence_theorem", "newell_normal").
    pub method: &'static str,
}

impl ComparisonPolicy {
    /// Check whether two values are within policy.
    pub fn within(&self, a: f64, b: f64) -> bool {
        let diff = (a - b).abs();
        let scale = a.abs().max(b.abs());
        diff <= self.abs_tol || diff <= self.rel_tol * scale
    }
}

/// Assert two f64 values are equal within the given `ComparisonPolicy`.
///
/// On failure, the panic message includes: the two values, the deviation,
/// both tolerance bounds, the algorithm name, and the caller-provided context.
/// This makes every geometric test failure fully auditable.
pub fn assert_geo_eq(a: f64, b: f64, policy: &ComparisonPolicy, context: &str) {
    if !policy.within(a, b) {
        let diff = (a - b).abs();
        let scale = a.abs().max(b.abs());
        panic!(
            "Geometric comparison FAILED [{context}]\n\
             \n  method:   {method}\n\
               computed: {a:.15e}\n\
               expected: {b:.15e}\n\
               abs_diff: {diff:.6e}\n\
               abs_tol:  {abs_tol:.6e} ({abs_ok})\n\
               rel_diff: {rel_diff:.6e}\n\
               rel_tol:  {rel_tol:.6e} ({rel_ok})\n\
               scale:    {scale:.6e}",
            method = policy.method,
            abs_tol = policy.abs_tol,
            abs_ok = if diff <= policy.abs_tol { "PASS" } else { "FAIL" },
            rel_diff = if scale > 0.0 { diff / scale } else { f64::INFINITY },
            rel_tol = policy.rel_tol,
            rel_ok = if diff <= policy.rel_tol * scale { "PASS" } else { "FAIL" },
        );
    }
}

/// Assert two `[f64; 3]` vectors are component-wise equal within policy.
pub fn assert_geo_eq_3d(a: [f64; 3], b: [f64; 3], policy: &ComparisonPolicy, context: &str) {
    for (i, axis) in ["x", "y", "z"].iter().enumerate() {
        assert_geo_eq(a[i], b[i], policy, &format!("{context} [{axis}]"));
    }
}

// ── Standard Policies ──────────────────────────────────────────────────────

/// Exact planar geometry — axis-aligned BSP primitives with no rounding.
///
/// Appropriate for volumes/normals of axis-aligned cubes/blocks where
/// the divergence theorem involves only exact integer-representable products.
pub fn exact_planar() -> ComparisonPolicy {
    ComparisonPolicy {
        abs_tol: 1e-14,
        rel_tol: 1e-15,
        method: "exact_planar",
    }
}

/// Analytical reference — comparing against a formula with irrational
/// constants (√3, φ, π). Floating point accumulation is expected.
///
/// Appropriate for dodecahedron volume, hexagonal prism area, etc.
pub fn analytical_reference() -> ComparisonPolicy {
    ComparisonPolicy {
        abs_tol: 1e-10,
        rel_tol: 1e-12,
        method: "analytical_reference",
    }
}

/// Volume invariance — asserting that a topology mutation did not change volume.
///
/// Uses the same tolerance as the existing `VOLUME_TOL` (1e-10) to maintain
/// backward compatibility with existing volume oracle tests.
pub fn volume_invariance() -> ComparisonPolicy {
    ComparisonPolicy {
        abs_tol: 1e-10,
        rel_tol: 1e-14,
        method: "volume_invariance",
    }
}

/// Normal magnitude — asserting ‖n‖ ≈ 1.0 for unit normals.
pub fn unit_normal() -> ComparisonPolicy {
    ComparisonPolicy {
        abs_tol: 1e-12,
        rel_tol: 1e-14,
        method: "unit_normal",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn within_abs_tolerance() {
        let p = exact_planar();
        assert!(p.within(1.0, 1.0 + 1e-15));
        assert!(!p.within(1.0, 1.0 + 1e-10));
    }

    #[test]
    fn within_rel_tolerance() {
        let p = analytical_reference();
        // 1e6 * 1e-12 = 1e-6 relative band
        assert!(p.within(1e6, 1e6 + 1e-7));
        assert!(!p.within(1e6, 1e6 + 1.0));
    }

    #[test]
    fn zero_values_use_abs() {
        let p = exact_planar();
        assert!(p.within(0.0, 1e-15));
        assert!(!p.within(0.0, 1e-10));
    }

    #[test]
    #[should_panic(expected = "Geometric comparison FAILED")]
    fn assert_geo_eq_panics_on_mismatch() {
        let p = exact_planar();
        assert_geo_eq(1.0, 2.0, &p, "test_mismatch");
    }
}
