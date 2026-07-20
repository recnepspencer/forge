use crate::identity::hash_parts;

use super::closeout_receipts::ReceiptSurfaceEvidence;

pub(super) struct EffectExecutionCloseoutSlopes {
    normalization: String,
    eligibility: String,
    lowering: String,
    execution: String,
    receipt_materialization: String,
    envelope_materialization: String,
    support_lookup: String,
}

impl EffectExecutionCloseoutSlopes {
    pub(super) fn normalization(&self) -> &str {
        &self.normalization
    }

    pub(super) fn eligibility(&self) -> &str {
        &self.eligibility
    }

    pub(super) fn lowering(&self) -> &str {
        &self.lowering
    }

    pub(super) fn execution(&self) -> &str {
        &self.execution
    }

    pub(super) fn receipt_materialization(&self) -> &str {
        &self.receipt_materialization
    }

    pub(super) fn envelope_materialization(&self) -> &str {
        &self.envelope_materialization
    }

    pub(super) fn support_lookup(&self) -> &str {
        &self.support_lookup
    }
}

pub(super) fn build_closeout_slopes(
    mutation: &ReceiptSurfaceEvidence,
    support_discovery_digest: &str,
    support_counter_digest: &str,
) -> EffectExecutionCloseoutSlopes {
    EffectExecutionCloseoutSlopes {
        normalization: stage_digest(
            "effect_normalization_slope_digest",
            &mutation.normalized_digest,
            &mutation.normalization_counter_digest,
        ),
        eligibility: stage_digest(
            "effect_eligibility_slope_digest",
            &mutation.eligibility_digest,
            &mutation.eligibility_counter_digest,
        ),
        lowering: stage_digest(
            "effect_lowering_slope_digest",
            &mutation.lowered_digest,
            &mutation.lowering_counter_digest,
        ),
        execution: stage_digest(
            "effect_execution_slope_digest",
            &mutation.receipt_digest,
            &mutation.execution_counter_digest,
        ),
        receipt_materialization: stage_digest(
            "effect_receipt_materialization_slope_digest",
            &mutation.transition_digest,
            &mutation.execution_counter_digest,
        ),
        envelope_materialization: stage_digest(
            "effect_envelope_materialization_slope_digest",
            &mutation.envelope_digest,
            &mutation.envelope_counter_digest,
        ),
        support_lookup: stage_digest(
            "effect_support_lookup_slope_digest",
            support_discovery_digest,
            support_counter_digest,
        ),
    }
}

fn stage_digest(stage: &str, artifact_digest: &str, counter_digest: &str) -> String {
    hash_parts(&[
        format!("stage:{stage}"),
        format!("artifact:{artifact_digest}"),
        format!("counters:{counter_digest}"),
    ])
}
