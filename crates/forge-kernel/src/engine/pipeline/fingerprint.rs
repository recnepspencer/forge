//! Pipeline fingerprint hashing.
//!
//! DOMAIN: Computes a deterministic fingerprint of the pipeline's input state.
//! Used for change detection (hash_before/hash_after on `OperationResult`),
//! NOT as a cache key. Configurable via `FingerprintDetail`.
//!
//! INVARIANTS:
//! - Commutative over inputs (order-independent via `wrapping_add`)
//! - Deterministic for same inputs + config + detail level
//! - Geometry hashing delegates to `SolidEnvelope::full_fingerprint()`

use std::collections::HashMap;

use forge_signal::facade::NodeId;

use crate::configuration::facade::FingerprintDetail;
use crate::engine::contracts::contract::ConditioningMode;
use crate::engine::output::solid_envelope::SolidEnvelope;

/// Compute a deterministic pipeline fingerprint from all input state.
///
/// Uses `wrapping_add` for combination — **commutative** (a + b = b + a),
/// making the hash independent of `HashMap` iteration order. This is
/// intentional: a feature taking `(target, tool)` produces the same hash
/// regardless of which input is iterated first.
///
/// Trade-off: commutative hashing can't distinguish permuted inputs
/// (e.g., `hash(A, B) == hash(B, A)`). Acceptable because this is for
/// change detection, not input identity — the feature tree structure
/// already encodes which NodeId is "target" vs "tool".
pub fn compute_pipeline_fingerprint(
    inputs: &HashMap<NodeId, SolidEnvelope>,
    feature_kind: &str,
    conditioning_mode: ConditioningMode,
    spatial_tolerance: f64,
    model_scale_mm: f64,
    min_edge_length: f64,
    detail: FingerprintDetail,
) -> u128 {
    let mut hash: u128 = 0;

    // ── Per-envelope hash (commutative via wrapping_add) ─────────────
    // Delegates to SolidEnvelope — single source of truth for
    // topology and geometry hashing.
    for output in inputs.values() {
        let envelope_hash = match detail {
            FingerprintDetail::Standard => output.topology_fingerprint(),
            FingerprintDetail::Full => output.full_fingerprint(),
        };
        hash = hash.wrapping_add(envelope_hash);
    }

    // ── Feature kind ─────────────────────────────────────────────────
    for byte in feature_kind.as_bytes() {
        hash = hash.wrapping_mul(31).wrapping_add(*byte as u128);
    }

    // ── Conditioning mode discriminant ───────────────────────────────
    let mode_tag: u8 = match conditioning_mode {
        ConditioningMode::None => 0,
        ConditioningMode::UnaryAnalysis => 1,
        ConditioningMode::BinaryAnalysis => 2,
    };
    hash = hash.wrapping_mul(31).wrapping_add(mode_tag as u128);

    // ── Key tolerance values ─────────────────────────────────────────
    hash = hash.wrapping_mul(31).wrapping_add(spatial_tolerance.to_bits() as u128);
    hash = hash.wrapping_mul(31).wrapping_add(model_scale_mm.to_bits() as u128);
    hash = hash.wrapping_mul(31).wrapping_add(min_edge_length.to_bits() as u128);

    hash
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::facade::GeometryStore;
    use forge_topo::transactions::TopologyState;

    #[test]
    fn same_inputs_same_fingerprint() {
        let inputs = HashMap::new();
        let a = compute_pipeline_fingerprint(
            &inputs, "make_cube", ConditioningMode::None,
            1e-7, 1.0, 1e-6, FingerprintDetail::Standard,
        );
        let b = compute_pipeline_fingerprint(
            &inputs, "make_cube", ConditioningMode::None,
            1e-7, 1.0, 1e-6, FingerprintDetail::Standard,
        );
        assert_eq!(a, b);
    }

    #[test]
    fn different_feature_kind_different_fingerprint() {
        let inputs = HashMap::new();
        let a = compute_pipeline_fingerprint(
            &inputs, "make_cube", ConditioningMode::None,
            1e-7, 1.0, 1e-6, FingerprintDetail::Standard,
        );
        let b = compute_pipeline_fingerprint(
            &inputs, "boolean", ConditioningMode::None,
            1e-7, 1.0, 1e-6, FingerprintDetail::Standard,
        );
        assert_ne!(a, b);
    }

    #[test]
    fn different_conditioning_different_fingerprint() {
        let inputs = HashMap::new();
        let a = compute_pipeline_fingerprint(
            &inputs, "boolean", ConditioningMode::None,
            1e-7, 1.0, 1e-6, FingerprintDetail::Standard,
        );
        let b = compute_pipeline_fingerprint(
            &inputs, "boolean", ConditioningMode::BinaryAnalysis,
            1e-7, 1.0, 1e-6, FingerprintDetail::Standard,
        );
        assert_ne!(a, b);
    }

    #[test]
    fn different_tolerance_different_fingerprint() {
        let inputs = HashMap::new();
        let a = compute_pipeline_fingerprint(
            &inputs, "make_cube", ConditioningMode::None,
            1e-7, 1.0, 1e-6, FingerprintDetail::Standard,
        );
        let b = compute_pipeline_fingerprint(
            &inputs, "make_cube", ConditioningMode::None,
            1e-6, 1.0, 1e-6, FingerprintDetail::Standard,
        );
        assert_ne!(a, b);
    }
}
