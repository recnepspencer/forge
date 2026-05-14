use crate::identity::hash_parts;

use super::closeout_receipts::ReceiptSurfaceEvidence;

pub(super) struct EffectExecutionCloseoutAudits {
    compile_fail_boundary_digest: String,
    proof_shape_digest: String,
    phase_progression_digest: String,
}

impl EffectExecutionCloseoutAudits {
    pub(super) fn compile_fail_boundary_digest(&self) -> &str {
        &self.compile_fail_boundary_digest
    }

    pub(super) fn proof_shape_digest(&self) -> &str {
        &self.proof_shape_digest
    }

    pub(super) fn phase_progression_digest(&self) -> &str {
        &self.phase_progression_digest
    }
}

pub(super) fn build_closeout_audits(
    mutation: &ReceiptSurfaceEvidence,
    writeback: &ReceiptSurfaceEvidence,
    batch: &ReceiptSurfaceEvidence,
    seeded_bundle_digest: &str,
    phase4_bundle_digest: &str,
) -> EffectExecutionCloseoutAudits {
    let compile_fail_boundary_digest = boundary_audit_digest();
    let proof_shape_digest = hash_parts(
        &std::iter::once(format!("boundary:{compile_fail_boundary_digest}"))
            .chain(proof_shape_parts("mutation", mutation))
            .chain(proof_shape_parts("writeback", writeback))
            .chain(proof_shape_parts("batch", batch))
            .collect::<Vec<_>>(),
    );
    let phase_progression_digest = hash_parts(&[
        format!("seeded:{seeded_bundle_digest}"),
        format!("phase4:{phase4_bundle_digest}"),
        proof_shape_digest.clone(),
    ]);
    EffectExecutionCloseoutAudits {
        compile_fail_boundary_digest,
        proof_shape_digest,
        phase_progression_digest,
    }
}

fn proof_shape_parts<'a>(
    label: &'static str,
    evidence: &'a ReceiptSurfaceEvidence,
) -> impl Iterator<Item = String> + 'a {
    [
        format!("{label}:raw:{}", evidence.raw_digest),
        format!("{label}:normalized:{}", evidence.normalized_digest),
        format!("{label}:eligibility:{}", evidence.eligibility_digest),
        format!("{label}:plan:{}", evidence.plan_digest),
        format!("{label}:lowered:{}", evidence.lowered_digest),
        format!("{label}:receipt:{}", evidence.receipt_digest),
        format!("{label}:envelope:{}", evidence.envelope_digest),
    ]
    .into_iter()
}

fn boundary_audit_digest() -> String {
    hash_parts(&[
        "execution:effect_lifecycle_execution_rejects_raw_intent".to_string(),
        "execution:effect_lifecycle_execution_rejects_normalized_intent".to_string(),
        "execution:effect_lifecycle_execution_rejects_admitted_effect".to_string(),
        "execution:effect_lifecycle_execution_rejects_authority_scoped_plan".to_string(),
        "lowering:effect_lifecycle_lowering_rejects_raw_intent".to_string(),
        "lowering:effect_lifecycle_lowering_rejects_normalized_intent".to_string(),
        "lowering:effect_lifecycle_lowered_execution_plan_constructor_private".to_string(),
        "support:effect_execution_receipt_constructor_private".to_string(),
        "support:self_describing_effect_envelope_constructor_private".to_string(),
        "support:effect_execution_certification_bundle_constructor_private".to_string(),
    ])
}
