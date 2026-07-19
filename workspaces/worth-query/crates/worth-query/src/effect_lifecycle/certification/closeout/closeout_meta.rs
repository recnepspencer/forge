use crate::identity::hash_parts;

use super::closeout_artifacts::{
    EffectExecutionCertificationLane, EffectExecutionCertificationRow,
};
use super::closeout_audits::EffectExecutionCloseoutAudits;
use super::closeout_dx::CloseoutDxEvidence;
use super::closeout_receipts::ReceiptSurfaceEvidence;
use super::closeout_slopes::EffectExecutionCloseoutSlopes;

pub(super) struct CloseoutMetaRows {
    pub(super) proof_shape: EffectExecutionCertificationRow,
    pub(super) performance: EffectExecutionCertificationRow,
}

pub(super) fn build_closeout_meta_rows(
    audits: &EffectExecutionCloseoutAudits,
    slopes: &EffectExecutionCloseoutSlopes,
    mutation: &ReceiptSurfaceEvidence,
    writeback: &ReceiptSurfaceEvidence,
    batch: &ReceiptSurfaceEvidence,
    dx: &CloseoutDxEvidence,
) -> CloseoutMetaRows {
    let proof_shape_counters = mutation
        .counters
        .combine(&writeback.counters)
        .combine(&batch.counters);
    let performance_counters = proof_shape_counters.combine(&dx.support_lookup_counters);

    CloseoutMetaRows {
        proof_shape: EffectExecutionCertificationRow::new(
            EffectExecutionCertificationLane::ProofShapeSurface,
            hash_parts(&[
                audits.proof_shape_digest().to_string(),
                audits.phase_progression_digest().to_string(),
            ]),
            format!(
                "proof_shape:{}|phase_progression:{}",
                audits.proof_shape_digest(),
                audits.phase_progression_digest()
            ),
            &proof_shape_counters,
            None,
        ),
        performance: EffectExecutionCertificationRow::new(
            EffectExecutionCertificationLane::PerformanceSurface,
            hash_parts(&[
                slopes.normalization().to_string(),
                slopes.eligibility().to_string(),
                slopes.lowering().to_string(),
                slopes.execution().to_string(),
                slopes.receipt_materialization().to_string(),
                slopes.envelope_materialization().to_string(),
                slopes.support_lookup().to_string(),
            ]),
            format!(
                "normalization:{}|eligibility:{}|lowering:{}|execution:{}|receipt:{}|envelope:{}|support:{}",
                slopes.normalization(),
                slopes.eligibility(),
                slopes.lowering(),
                slopes.execution(),
                slopes.receipt_materialization(),
                slopes.envelope_materialization(),
                slopes.support_lookup()
            ),
            &performance_counters,
            None,
        ),
    }
}
