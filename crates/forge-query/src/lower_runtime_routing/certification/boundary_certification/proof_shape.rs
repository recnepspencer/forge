use crate::identity::hash_parts;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryLowerRuntimeProofShapeViolation {
    DirectLowerRuntimeImportBypass,
    RoutePlanOrReceiptOmission,
    DeferredNeighborMasquerade,
    SpecialistDebtSurvival,
    DownstreamBoundaryLeak,
}

impl ForgeQueryLowerRuntimeProofShapeViolation {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::DirectLowerRuntimeImportBypass => "direct-lower-runtime-import-bypass",
            Self::RoutePlanOrReceiptOmission => "route-plan-or-receipt-omission",
            Self::DeferredNeighborMasquerade => "deferred-neighbor-masquerade",
            Self::SpecialistDebtSurvival => "specialist-debt-survival",
            Self::DownstreamBoundaryLeak => "downstream-boundary-leak",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryLowerRuntimeProofShapeEnforcement {
    CompileFailFixture,
    CertificationRuntimeAudit,
    NonBypassAudit,
}

impl ForgeQueryLowerRuntimeProofShapeEnforcement {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::CompileFailFixture => "compile-fail-fixture",
            Self::CertificationRuntimeAudit => "certification-runtime-audit",
            Self::NonBypassAudit => "non-bypass-audit",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryLowerRuntimeProofShapeAuditRow {
    violation: ForgeQueryLowerRuntimeProofShapeViolation,
    attempted_shortcut: &'static str,
    required_prior_artifact: &'static str,
    rejected_artifact: &'static str,
    enforcement: ForgeQueryLowerRuntimeProofShapeEnforcement,
    enforcement_proof: &'static str,
    row_digest: String,
}

impl ForgeQueryLowerRuntimeProofShapeAuditRow {
    fn new(
        violation: ForgeQueryLowerRuntimeProofShapeViolation,
        attempted_shortcut: &'static str,
        required_prior_artifact: &'static str,
        rejected_artifact: &'static str,
        enforcement: ForgeQueryLowerRuntimeProofShapeEnforcement,
        enforcement_proof: &'static str,
    ) -> Self {
        let row_digest = hash_parts(&[
            "lower_runtime_routing_proof_shape_row_v1".to_string(),
            format!("violation:{}", violation.as_str()),
            format!("attempted_shortcut:{attempted_shortcut}"),
            format!("required_prior_artifact:{required_prior_artifact}"),
            format!("rejected_artifact:{rejected_artifact}"),
            format!("enforcement:{}", enforcement.as_str()),
            format!("proof:{enforcement_proof}"),
        ]);
        Self {
            violation,
            attempted_shortcut,
            required_prior_artifact,
            rejected_artifact,
            enforcement,
            enforcement_proof,
            row_digest,
        }
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }

    pub fn required_prior_artifact(&self) -> &'static str {
        self.required_prior_artifact
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryLowerRuntimeProofShapeAudit {
    rows: Vec<ForgeQueryLowerRuntimeProofShapeAuditRow>,
    proof_shape_digest: String,
    phase_progression_digest: String,
}

impl ForgeQueryLowerRuntimeProofShapeAudit {
    fn new(rows: Vec<ForgeQueryLowerRuntimeProofShapeAuditRow>) -> Self {
        let proof_shape_digest = hash_parts(
            &rows
                .iter()
                .map(|row| row.row_digest().to_string())
                .collect::<Vec<_>>(),
        );
        let phase_progression_digest = hash_parts(&[
            "lower_runtime_routing_phase_progression_v1".to_string(),
            proof_shape_digest.clone(),
            rows.iter()
                .map(|row| row.required_prior_artifact())
                .collect::<Vec<_>>()
                .join(">"),
        ]);
        Self {
            rows,
            proof_shape_digest,
            phase_progression_digest,
        }
    }

    pub fn rows(&self) -> &[ForgeQueryLowerRuntimeProofShapeAuditRow] {
        &self.rows
    }

    pub fn proof_shape_digest(&self) -> &str {
        &self.proof_shape_digest
    }

    pub fn phase_progression_digest(&self) -> &str {
        &self.phase_progression_digest
    }
}

pub fn forge_query_lower_runtime_proof_shape_audit() -> ForgeQueryLowerRuntimeProofShapeAudit {
    use ForgeQueryLowerRuntimeProofShapeEnforcement::*;
    use ForgeQueryLowerRuntimeProofShapeViolation::*;

    ForgeQueryLowerRuntimeProofShapeAudit::new(vec![
        row(
            DirectLowerRuntimeImportBypass,
            "ordinary caller or Query module imports bridge/relational/signal facade directly",
            "public routed facade plus certified non-bypass boundary",
            "direct lower-runtime import path",
            NonBypassAudit,
            "certify_lower_runtime_non_bypass",
        ),
        row(
            RoutePlanOrReceiptOmission,
            "covered seam returns loose operational token without route/receipt lifecycle",
            "typed capability request, eligibility, route/handoff, receipt, and envelope",
            "weak unit/string seam",
            CertificationRuntimeAudit,
            "certify_lower_runtime_routing",
        ),
        row(
            DeferredNeighborMasquerade,
            "store/temporal/async neighbor claims admitted support",
            "closeout registry row plus deferred support posture",
            "pretend-admitted future neighbor",
            CertificationRuntimeAudit,
            "certify_lower_runtime_routing",
        ),
        row(
            SpecialistDebtSurvival,
            "former frontier/writeback specialist seam survives as compatibility debt",
            "named lower-runtime contract plus adapter/reuse classification",
            "compatibility-debt seam",
            CertificationRuntimeAudit,
            "certify_lower_runtime_routing",
        ),
        row(
            DownstreamBoundaryLeak,
            "downstream Query-facing domain imports lower-runtime facades outside the runtime boundary subtree",
            "declared downstream runtime-boundary subtree only",
            "out-of-subtree lower-runtime import",
            CompileFailFixture,
            "phase_boundaries_lower_runtime_routing_compile_fail",
        ),
    ])
}

pub fn forge_query_lower_runtime_proof_shape_digest() -> String {
    forge_query_lower_runtime_proof_shape_audit()
        .proof_shape_digest()
        .to_string()
}

pub fn forge_query_lower_runtime_phase_progression_digest() -> String {
    forge_query_lower_runtime_proof_shape_audit()
        .phase_progression_digest()
        .to_string()
}

fn row(
    violation: ForgeQueryLowerRuntimeProofShapeViolation,
    attempted_shortcut: &'static str,
    required_prior_artifact: &'static str,
    rejected_artifact: &'static str,
    enforcement: ForgeQueryLowerRuntimeProofShapeEnforcement,
    enforcement_proof: &'static str,
) -> ForgeQueryLowerRuntimeProofShapeAuditRow {
    ForgeQueryLowerRuntimeProofShapeAuditRow::new(
        violation,
        attempted_shortcut,
        required_prior_artifact,
        rejected_artifact,
        enforcement,
        enforcement_proof,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn proof_shape_audit_tracks_phase_progression() {
        let audit = forge_query_lower_runtime_proof_shape_audit();

        assert_eq!(audit.rows().len(), 5);
        assert!(!audit.proof_shape_digest().is_empty());
        assert!(!audit.phase_progression_digest().is_empty());
    }
}
