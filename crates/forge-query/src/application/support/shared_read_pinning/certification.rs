use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag,
};

use super::closure::{
    ForgeQuerySharedReadPinningBoundaryClosure, ForgeQuerySharedReadPinningBoundaryEvidence,
};
use super::evidence::{
    ForgeQuerySharedReadPinningCounterEvidence, ForgeQuerySharedReadPinningHostileMatrixEvidence,
    ForgeQuerySharedReadPinningInventoryEvidence, ForgeQuerySharedReadPortabilityEvidence,
    ForgeQuerySharedReadStaleBasisDenialEvidence,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQuerySharedReadPinningCertification {
    inventory: ForgeQuerySharedReadPinningInventoryEvidence,
    hostile_matrix: ForgeQuerySharedReadPinningHostileMatrixEvidence,
    portability: ForgeQuerySharedReadPortabilityEvidence,
    stale_basis_denial: ForgeQuerySharedReadStaleBasisDenialEvidence,
    counters: ForgeQuerySharedReadPinningCounterEvidence,
    closure: ForgeQuerySharedReadPinningBoundaryClosure,
    artifact_digest: String,
    failure_digest: String,
}

impl ForgeQuerySharedReadPinningCertification {
    pub fn from_evidence(
        inventory: ForgeQuerySharedReadPinningInventoryEvidence,
        hostile_matrix: ForgeQuerySharedReadPinningHostileMatrixEvidence,
        portability: ForgeQuerySharedReadPortabilityEvidence,
        stale_basis_denial: ForgeQuerySharedReadStaleBasisDenialEvidence,
        counters: ForgeQuerySharedReadPinningCounterEvidence,
    ) -> Self {
        let closure_evidence = ForgeQuerySharedReadPinningBoundaryEvidence::new(
            inventory.total_failure_count(),
            counters.residue_count(),
            hostile_matrix.certified(),
            portability.proven_by_scoped_thread(),
            stale_basis_denial.proven_by_typed_denial(),
        );
        let closure =
            ForgeQuerySharedReadPinningBoundaryClosure::derive_from_evidence(&closure_evidence);
        let failure_digest =
            forge_query_evidence_identity(ForgeQueryEvidenceScope::ApplicationSupportReport)
                .field_usize(
                    ForgeQueryEvidenceTag::new("inventory_failure_count"),
                    inventory.total_failure_count(),
                )
                .field_usize(
                    ForgeQueryEvidenceTag::new("counter_residue_count"),
                    counters.residue_count(),
                )
                .field_bool(
                    ForgeQueryEvidenceTag::new("hostile_matrix_certified"),
                    hostile_matrix.certified(),
                )
                .field_bool(
                    ForgeQueryEvidenceTag::new("portability_proven"),
                    portability.proven_by_scoped_thread(),
                )
                .field_bool(
                    ForgeQueryEvidenceTag::new("stale_basis_denial_proven"),
                    stale_basis_denial.proven_by_typed_denial(),
                )
                .seal()
                .as_str()
                .to_string();
        let artifact_digest = forge_query_evidence_identity(
            ForgeQueryEvidenceScope::RuntimeHostileCertificationArtifact,
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("shared_read_pinning_inventory_digest"),
            inventory.inventory_digest(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("hostile_matrix_digest"),
            hostile_matrix.matrix_digest(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("send_sync_proof_digest"),
            portability.scoped_thread_digest(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("stale_basis_denial_digest"),
            stale_basis_denial.typed_denial_digest(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("counter_digest"),
            counters.counter_digest(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("closure_digest"),
            closure.closure_digest(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("failure_digest"),
            &failure_digest,
        )
        .seal()
        .as_str()
        .to_string();
        Self {
            inventory,
            hostile_matrix,
            portability,
            stale_basis_denial,
            counters,
            closure,
            artifact_digest,
            failure_digest,
        }
    }

    pub fn support_gate_required() -> Self {
        Self::from_evidence(
            ForgeQuerySharedReadPinningInventoryEvidence::new(
                0,
                0,
                "support-matrix-requires-runtime-pinning-certification",
            ),
            ForgeQuerySharedReadPinningHostileMatrixEvidence::new(false, ""),
            ForgeQuerySharedReadPortabilityEvidence::proven(""),
            ForgeQuerySharedReadStaleBasisDenialEvidence::proven(""),
            ForgeQuerySharedReadPinningCounterEvidence::new(0, 0, 0, 0, 0),
        )
    }

    pub fn closure(&self) -> &ForgeQuerySharedReadPinningBoundaryClosure {
        &self.closure
    }

    #[allow(dead_code)]
    pub fn artifact_digest(&self) -> &str {
        &self.artifact_digest
    }

    #[allow(dead_code)]
    pub fn failure_digest(&self) -> &str {
        &self.failure_digest
    }
}
