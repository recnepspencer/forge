use crate::identity::hash_parts;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BasisLifecycleProofShapeViolation {
    PhaseSkipping,
    RawIdentifierSubstitution,
    StaleProofReuse,
    OperationLaneWORTHry,
    WorthdLowerRuntimeAuthority,
}

impl BasisLifecycleProofShapeViolation {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::PhaseSkipping => "phase_skipping",
            Self::RawIdentifierSubstitution => "raw_identifier_substitution",
            Self::StaleProofReuse => "stale_proof_reuse",
            Self::OperationLaneWORTHry => "operation_lane_WORTHry",
            Self::WorthdLowerRuntimeAuthority => "worthd_lower_runtime_authority",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BasisLifecycleProofShapeEnforcement {
    CompileFailFixture,
    RuntimeDenialTest,
    BoundaryAudit,
}

impl BasisLifecycleProofShapeEnforcement {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::CompileFailFixture => "compile_fail_fixture",
            Self::RuntimeDenialTest => "runtime_denial_test",
            Self::BoundaryAudit => "boundary_audit",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BasisLifecycleProofShapeAuditRow {
    violation: BasisLifecycleProofShapeViolation,
    attempted_shortcut: &'static str,
    required_prior_artifact: &'static str,
    rejected_artifact: &'static str,
    enforcement: BasisLifecycleProofShapeEnforcement,
    enforcement_proof: &'static str,
    row_digest: String,
}

impl BasisLifecycleProofShapeAuditRow {
    fn new(
        violation: BasisLifecycleProofShapeViolation,
        attempted_shortcut: &'static str,
        required_prior_artifact: &'static str,
        rejected_artifact: &'static str,
        enforcement: BasisLifecycleProofShapeEnforcement,
        enforcement_proof: &'static str,
    ) -> Self {
        let row_digest = hash_parts(&[
            "basis_lifecycle_proof_shape_row_v1".to_string(),
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

    pub fn violation(&self) -> BasisLifecycleProofShapeViolation {
        self.violation
    }

    pub fn attempted_shortcut(&self) -> &'static str {
        self.attempted_shortcut
    }

    pub fn required_prior_artifact(&self) -> &'static str {
        self.required_prior_artifact
    }

    pub fn rejected_artifact(&self) -> &'static str {
        self.rejected_artifact
    }

    pub fn enforcement(&self) -> BasisLifecycleProofShapeEnforcement {
        self.enforcement
    }

    pub fn enforcement_proof(&self) -> &'static str {
        self.enforcement_proof
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BasisLifecycleProofShapeAudit {
    rows: Vec<BasisLifecycleProofShapeAuditRow>,
    proof_shape_digest: String,
    phase_progression_digest: String,
}

impl BasisLifecycleProofShapeAudit {
    fn new(rows: Vec<BasisLifecycleProofShapeAuditRow>) -> Self {
        let proof_shape_digest = hash_parts(
            &rows
                .iter()
                .map(|row| row.row_digest().to_string())
                .collect::<Vec<_>>(),
        );
        let phase_progression_digest = hash_parts(&[
            "basis_lifecycle_phase_progression_v1".to_string(),
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

    pub fn rows(&self) -> &[BasisLifecycleProofShapeAuditRow] {
        &self.rows
    }

    pub fn proof_shape_digest(&self) -> &str {
        &self.proof_shape_digest
    }

    pub fn phase_progression_digest(&self) -> &str {
        &self.phase_progression_digest
    }

    pub fn row_for(
        &self,
        violation: BasisLifecycleProofShapeViolation,
    ) -> Option<&BasisLifecycleProofShapeAuditRow> {
        self.rows.iter().find(|row| row.violation() == violation)
    }
}

pub fn basis_lifecycle_proof_shape_audit() -> BasisLifecycleProofShapeAudit {
    use BasisLifecycleProofShapeEnforcement::*;
    use BasisLifecycleProofShapeViolation::*;
    BasisLifecycleProofShapeAudit::new(vec![
        row(
            PhaseSkipping,
            "RawBasisIntent or NormalizedBasisIntent passed directly to scoped use",
            "AdmittedBasisCapability then operation-scoped basis",
            "unscoped lifecycle draft",
            CompileFailFixture,
            "basis_lifecycle_dx_draft_is_not_scoped_proof",
        ),
        row(
            RawIdentifierSubstitution,
            "raw branch, snapshot, preview, tenant, policy, or runtime token",
            "proof-bearing admitted or scoped basis capability",
            "plain identifier or digest",
            BoundaryAudit,
            "basis_lifecycle_public_boundary_audit",
        ),
        row(
            StaleProofReuse,
            "scoped proof reused after lane or lower-runtime binding changes",
            "fresh scoped basis tied to current capability and evidence digest",
            "stale scoped basis digest",
            RuntimeDenialTest,
            "stale_runtime_snapshot_evidence_denies_at_readmission_boundary",
        ),
        row(
            OperationLaneWORTHry,
            "generic or caller-made lane witness",
            "sealed operation-specific lane witness",
            "Worthd operation lane",
            CompileFailFixture,
            "basis_lifecycle_lane_witness_constructor_private",
        ),
        row(
            WorthdLowerRuntimeAuthority,
            "caller-made lower-runtime authority witness",
            "facade-returned lower-runtime evidence plus scoped proof",
            "Worthd lower-runtime authority",
            CompileFailFixture,
            "basis_lifecycle_lower_runtime_evidence_constructor_private",
        ),
    ])
}

pub fn basis_lifecycle_proof_shape_audit_digest() -> String {
    basis_lifecycle_proof_shape_audit()
        .proof_shape_digest()
        .to_string()
}

pub fn basis_lifecycle_phase_progression_digest() -> String {
    basis_lifecycle_proof_shape_audit()
        .phase_progression_digest()
        .to_string()
}

fn row(
    violation: BasisLifecycleProofShapeViolation,
    attempted_shortcut: &'static str,
    required_prior_artifact: &'static str,
    rejected_artifact: &'static str,
    enforcement: BasisLifecycleProofShapeEnforcement,
    enforcement_proof: &'static str,
) -> BasisLifecycleProofShapeAuditRow {
    BasisLifecycleProofShapeAuditRow::new(
        violation,
        attempted_shortcut,
        required_prior_artifact,
        rejected_artifact,
        enforcement,
        enforcement_proof,
    )
}

#[cfg(test)]
mod tests;
