use worth_primitives::{truth_digest_parts, TruthDigestScope};

use worth_kernel::workload_composition::{
    PlanarBooleanBlockerEvidenceReceipt, PlanarBooleanDeclarationReceipt,
    PlanarBooleanOperandPairConstructionReceipt, PlanarBooleanSupportReceipt,
    WorkloadCompositionError, WorthWorkload,
};
use worth_spatial::facade::workload_vocabulary::CompleteWorkloadEvidenceLedger;

use super::{anti_theatre_guards, PlanarBoolean7_0CloseoutError};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct PlanarBoolean7_0EvidenceProof {
    proof_digest: String,
    entry_boundary_digest: String,
    readiness_basis_digest: String,
    declaration_digest: String,
    operand_pair_identity: String,
    blocker_digest: String,
}

impl PlanarBoolean7_0EvidenceProof {
    pub(super) fn certify(
        admitted_workload: &WorthWorkload,
        declaration: &PlanarBooleanDeclarationReceipt,
        route: &PlanarBooleanSupportReceipt,
        pair_construction: &PlanarBooleanOperandPairConstructionReceipt,
        blocked_workload: &WorthWorkload,
        blocker: &PlanarBooleanBlockerEvidenceReceipt,
    ) -> Result<Self, WorkloadCompositionError> {
        admitted_workload.require_boolean_declaration_entry(declaration)?;
        admitted_workload.require_boolean_route_plan(route)?;
        admitted_workload.require_boolean_operand_pair_construction(pair_construction)?;
        blocked_workload.require_boolean_blocker_provenance(blocker)?;

        let entry_boundary_digest = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &[
                "planar-boolean-7-0-entry-boundary".to_string(),
                format!("basis:{}", declaration.readiness_basis_digest()),
                format!("declaration:{}", declaration.query_declaration_digest()),
                format!("route:{}", route.query_support_digest()),
                format!("pair:{}", pair_construction.construction_digest()),
            ],
        );
        let proof_digest = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &[
                "planar-boolean-7-0-evidence-proof".to_string(),
                format!("entry-boundary:{entry_boundary_digest}"),
                format!("blocker:{}", blocker.blocker_digest()),
            ],
        );

        Ok(Self {
            proof_digest,
            entry_boundary_digest,
            readiness_basis_digest: declaration.readiness_basis_digest().to_string(),
            declaration_digest: declaration.query_declaration_digest().to_string(),
            operand_pair_identity: pair_construction.operand_pair_identity().to_string(),
            blocker_digest: blocker.blocker_digest().to_string(),
        })
    }

    pub(super) fn proof_digest(&self) -> &str {
        &self.proof_digest
    }

    pub(super) fn entry_boundary_digest(&self) -> &str {
        &self.entry_boundary_digest
    }

    pub(super) fn readiness_basis_digest(&self) -> &str {
        &self.readiness_basis_digest
    }

    pub(super) fn declaration_digest(&self) -> &str {
        &self.declaration_digest
    }

    pub(super) fn operand_pair_identity(&self) -> &str {
        &self.operand_pair_identity
    }

    pub(super) fn blocker_digest(&self) -> &str {
        &self.blocker_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct PlanarBoolean7_0AntiTheatreProof {
    proof_digest: String,
    blocker_digest: String,
    pair_construction_digest: String,
}

impl PlanarBoolean7_0AntiTheatreProof {
    pub(super) fn certify(
        topology_ledger: &CompleteWorkloadEvidenceLedger,
        blocked_workload: &WorthWorkload,
        blocker: &PlanarBooleanBlockerEvidenceReceipt,
        admitted_workload: &WorthWorkload,
        pair_construction: &PlanarBooleanOperandPairConstructionReceipt,
        kernel_summary_fixture_identity: &str,
    ) -> Result<Self, PlanarBoolean7_0CloseoutError> {
        let topology_guard_identity =
            anti_theatre_guards::topology_guard_identity(topology_ledger)?;
        let blocker_guard_identity =
            anti_theatre_guards::blocker_guard_identity(blocked_workload, blocker)?;
        let catalog_guard_identity =
            anti_theatre_guards::catalog_guard_identity(admitted_workload, pair_construction)?;
        let proof_digest = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &[
                "planar-boolean-7-0-anti-theatre".to_string(),
                format!("topology-guard:{topology_guard_identity}"),
                format!("blocker-guard:{blocker_guard_identity}"),
                format!("catalog-guard:{catalog_guard_identity}"),
                format!("kernel-summary:{kernel_summary_fixture_identity}"),
            ],
        );
        Ok(Self {
            proof_digest,
            blocker_digest: blocker.blocker_digest().to_string(),
            pair_construction_digest: pair_construction.construction_digest().to_string(),
        })
    }

    pub(super) fn proof_digest(&self) -> &str {
        &self.proof_digest
    }

    pub(super) fn blocker_digest(&self) -> &str {
        &self.blocker_digest
    }

    pub(super) fn pair_construction_digest(&self) -> &str {
        &self.pair_construction_digest
    }
}
