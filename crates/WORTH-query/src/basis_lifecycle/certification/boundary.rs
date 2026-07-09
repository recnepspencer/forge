use crate::identity::hash_parts;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BasisLifecyclePublicBoundarySurface {
    BranchIdentifier,
    SnapshotIdentifier,
    PreviewIdentifier,
    TenantIdentifier,
    PolicyIdentifier,
    RuntimeSnapshotIdentifier,
    RawBasisIntentSubstitution,
    NormalizedIntentSubstitution,
}

impl BasisLifecyclePublicBoundarySurface {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::BranchIdentifier => "branch_identifier",
            Self::SnapshotIdentifier => "snapshot_identifier",
            Self::PreviewIdentifier => "preview_identifier",
            Self::TenantIdentifier => "tenant_identifier",
            Self::PolicyIdentifier => "policy_identifier",
            Self::RuntimeSnapshotIdentifier => "runtime_snapshot_identifier",
            Self::RawBasisIntentSubstitution => "raw_basis_intent_substitution",
            Self::NormalizedIntentSubstitution => "normalized_intent_substitution",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BasisLifecyclePublicBoundaryAuditRow {
    surface: BasisLifecyclePublicBoundarySurface,
    forbidden_token: &'static str,
    blocked_entrypoint: &'static str,
    required_capability: &'static str,
    enforcement_proof: &'static str,
    row_digest: String,
}

impl BasisLifecyclePublicBoundaryAuditRow {
    fn new(
        surface: BasisLifecyclePublicBoundarySurface,
        forbidden_token: &'static str,
        blocked_entrypoint: &'static str,
        required_capability: &'static str,
        enforcement_proof: &'static str,
    ) -> Self {
        let row_digest = hash_parts(&[
            "basis_lifecycle_public_boundary_row_v1".to_string(),
            format!("surface:{}", surface.as_str()),
            format!("forbidden_token:{forbidden_token}"),
            format!("blocked_entrypoint:{blocked_entrypoint}"),
            format!("required_capability:{required_capability}"),
            format!("proof:{enforcement_proof}"),
        ]);
        Self {
            surface,
            forbidden_token,
            blocked_entrypoint,
            required_capability,
            enforcement_proof,
            row_digest,
        }
    }

    pub fn surface(&self) -> BasisLifecyclePublicBoundarySurface {
        self.surface
    }

    pub fn forbidden_token(&self) -> &'static str {
        self.forbidden_token
    }

    pub fn blocked_entrypoint(&self) -> &'static str {
        self.blocked_entrypoint
    }

    pub fn required_capability(&self) -> &'static str {
        self.required_capability
    }

    pub fn enforcement_proof(&self) -> &'static str {
        self.enforcement_proof
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BasisLifecyclePublicBoundaryAudit {
    rows: Vec<BasisLifecyclePublicBoundaryAuditRow>,
    audit_digest: String,
}

impl BasisLifecyclePublicBoundaryAudit {
    fn new(rows: Vec<BasisLifecyclePublicBoundaryAuditRow>) -> Self {
        let audit_digest = hash_parts(
            &rows
                .iter()
                .map(|row| row.row_digest().to_string())
                .collect::<Vec<_>>(),
        );
        Self { rows, audit_digest }
    }

    pub fn rows(&self) -> &[BasisLifecyclePublicBoundaryAuditRow] {
        &self.rows
    }

    pub fn audit_digest(&self) -> &str {
        &self.audit_digest
    }

    pub fn row_for(
        &self,
        surface: BasisLifecyclePublicBoundarySurface,
    ) -> Option<&BasisLifecyclePublicBoundaryAuditRow> {
        self.rows.iter().find(|row| row.surface() == surface)
    }
}

pub fn basis_lifecycle_public_boundary_audit() -> BasisLifecyclePublicBoundaryAudit {
    use BasisLifecyclePublicBoundarySurface::*;
    BasisLifecyclePublicBoundaryAudit::new(vec![
        row(
            BranchIdentifier,
            "raw branch id",
            "read/mutation/replay/inspection/materialization/subscription",
            "Scoped*Basis",
            "facade_query_read_capability_has_no_query_context_basis_bundle",
        ),
        row(
            SnapshotIdentifier,
            "raw snapshot token",
            "read/replay/materialization",
            "ScopedObservationBasis or ScopedReplayBasis",
            "historical_materialization_metadata_is_not_query_basis_result_bundle",
        ),
        row(
            PreviewIdentifier,
            "raw preview label",
            "preview closeout or promotion",
            "ScopedPreviewCloseoutBasis",
            "facade_preview_capability_cannot_admit_workflow",
        ),
        row(
            TenantIdentifier,
            "raw tenant id",
            "policy or tenant-scoped execution",
            "AdmittedBasisCapability",
            "admitted_policy_tenant_context_constructor_private",
        ),
        row(
            PolicyIdentifier,
            "raw policy digest",
            "policy-scoped execution",
            "AdmittedBasisCapability",
            "policy_narrowing_requires_admitted_context",
        ),
        row(
            RuntimeSnapshotIdentifier,
            "raw runtime snapshot token",
            "lower-runtime readmission",
            "LowerRuntimeBasisEvidence plus ScopedBasisProof",
            "basis_lifecycle_dx_draft_is_not_scoped_proof",
        ),
        row(
            RawBasisIntentSubstitution,
            "RawBasisIntent",
            "scoped use or lower-runtime readmission",
            "NormalizedBasisIntent then AdmittedBasisCapability",
            "basis_lifecycle_dx_draft_is_not_scoped_proof",
        ),
        row(
            NormalizedIntentSubstitution,
            "NormalizedBasisIntent",
            "scoped use or lower-runtime readmission",
            "AdmittedBasisCapability",
            "basis_lifecycle_advisory_cannot_be_admitted",
        ),
    ])
}

pub fn basis_lifecycle_public_boundary_audit_digest() -> String {
    basis_lifecycle_public_boundary_audit()
        .audit_digest()
        .to_string()
}

fn row(
    surface: BasisLifecyclePublicBoundarySurface,
    forbidden_token: &'static str,
    blocked_entrypoint: &'static str,
    required_capability: &'static str,
    enforcement_proof: &'static str,
) -> BasisLifecyclePublicBoundaryAuditRow {
    BasisLifecyclePublicBoundaryAuditRow::new(
        surface,
        forbidden_token,
        blocked_entrypoint,
        required_capability,
        enforcement_proof,
    )
}

#[cfg(test)]
mod tests;
