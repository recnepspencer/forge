//! Pipeline fingerprint hashing.
//!
//! DOMAIN: Computes a deterministic fingerprint of the pipeline's input state.
//! Used for change detection (hash_before/hash_after on `OperationResult`),
//! NOT as a cache key. Configurable via `FingerprintDetail`.
//!
//! INVARIANTS:
//! - Commutative over inputs (order-independent via `wrapping_add`)
//! - Deterministic for same inputs + config + detail level
//! - Envelope hashing delegates to the output envelope type

use std::collections::HashMap;

use forge_signal::facade::NodeId;

use crate::configuration::facade::FingerprintDetail;
use crate::engine::contracts::contract::ConditioningMode;
use crate::engine::output::solid_envelope::SolidEnvelope;
use crate::engine::output::spec_envelope::SpecEnvelope;

type InfallibleFingerprintResult = Result<u128, core::convert::Infallible>;

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
    compute_commutative_pipeline_fingerprint(
        inputs.values().map(|output| match detail {
            FingerprintDetail::Standard => {
                InfallibleFingerprintResult::Ok(output.topology_fingerprint())
            }
            FingerprintDetail::Full => InfallibleFingerprintResult::Ok(output.full_fingerprint()),
        }),
        feature_kind,
        conditioning_mode,
        spatial_tolerance,
        model_scale_mm,
        min_edge_length,
    )
    .expect("solid envelope fingerprinting is infallible")
}

/// Compute a deterministic pipeline fingerprint from spec-backed input state.
pub fn compute_spec_pipeline_fingerprint(
    inputs: &HashMap<NodeId, SpecEnvelope>,
    feature_kind: &str,
    conditioning_mode: ConditioningMode,
    spatial_tolerance: f64,
    model_scale_mm: f64,
    min_edge_length: f64,
    detail: FingerprintDetail,
) -> Result<u128, forge_core::KernelError> {
    compute_commutative_pipeline_fingerprint(
        inputs.values().map(|output| output.fingerprint(detail)),
        feature_kind,
        conditioning_mode,
        spatial_tolerance,
        model_scale_mm,
        min_edge_length,
    )
}

fn compute_commutative_pipeline_fingerprint<I, E>(
    hashes: I,
    feature_kind: &str,
    conditioning_mode: ConditioningMode,
    spatial_tolerance: f64,
    model_scale_mm: f64,
    min_edge_length: f64,
) -> Result<u128, E>
where
    I: IntoIterator<Item = Result<u128, E>>,
{
    let mut hash: u128 = 0;

    for envelope_hash in hashes {
        hash = hash.wrapping_add(envelope_hash?);
    }

    for byte in feature_kind.as_bytes() {
        hash = hash.wrapping_mul(31).wrapping_add(*byte as u128);
    }

    let mode_tag: u8 = match conditioning_mode {
        ConditioningMode::None => 0,
        ConditioningMode::UnaryAnalysis => 1,
        ConditioningMode::BinaryAnalysis => 2,
    };
    hash = hash.wrapping_mul(31).wrapping_add(mode_tag as u128);
    hash = hash
        .wrapping_mul(31)
        .wrapping_add(spatial_tolerance.to_bits() as u128);
    hash = hash
        .wrapping_mul(31)
        .wrapping_add(model_scale_mm.to_bits() as u128);
    hash = hash
        .wrapping_mul(31)
        .wrapping_add(min_edge_length.to_bits() as u128);

    Ok(hash)
}
