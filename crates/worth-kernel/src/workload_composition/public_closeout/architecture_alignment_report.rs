use topology::certification::current_topology_public_closeout_alignment_summary;
use worth_primitives::{truth_digest_parts, TruthDigestScope};
use worth_spatial::certification::current_spatial_public_closeout_alignment_summary;

use super::public_closeout_types::{
    WorthTouchedGraphConflictPublicCloseoutError, WorthTouchedGraphConflictPublicCloseoutErrorKind,
};
use super::residue_chain::{
    WorthTouchedGraphConflictResidueBoundaryPosture, WorthTouchedGraphConflictResidueChain,
    WorthTouchedGraphConflictResidueRow,
};
use crate::workload_composition::deletion_closeout::{
    WorthTouchedGraphConflictDeletionCloseout, WorthTouchedGraphConflictDeletionDisposition,
    WorthTouchedGraphConflictDeletionLedgerRow,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthTouchedGraphConflictArchitectureAlignmentReportRow {
    surface_name: String,
    owner: String,
    blocker: String,
    removal_trigger: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthTouchedGraphConflictDeletionAlignmentRow {
    source_path: String,
    surface_name: String,
    blocker: String,
    removal_trigger: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthTouchedGraphConflictArchitectureAlignmentReport {
    topology_compiled_product_identity_digest: String,
    topology_equivalence_policy_identity_digest: String,
    spatial_compiled_product_identity_digest: String,
    spatial_equivalence_policy_identity_digest: String,
    reuse_decision_identity_digest: Option<String>,
    rebuild_denial_identity_digest: Option<String>,
    displaced_legacy_authority_rows: Vec<WorthTouchedGraphConflictDeletionAlignmentRow>,
    deleted_authority_rows: Vec<WorthTouchedGraphConflictDeletionAlignmentRow>,
    capped_deletion_rows: Vec<WorthTouchedGraphConflictDeletionAlignmentRow>,
    certification_only_fence_rows: Vec<WorthTouchedGraphConflictDeletionAlignmentRow>,
    capped_residue_rows: Vec<WorthTouchedGraphConflictArchitectureAlignmentReportRow>,
    ordinary_second_ontology_blockers: Vec<WorthTouchedGraphConflictArchitectureAlignmentReportRow>,
    report_digest: String,
}

pub(crate) fn build_architecture_alignment_report(
    deletion_closeout: &WorthTouchedGraphConflictDeletionCloseout,
    residue_chain: &WorthTouchedGraphConflictResidueChain,
) -> Result<
    WorthTouchedGraphConflictArchitectureAlignmentReport,
    WorthTouchedGraphConflictPublicCloseoutError,
> {
    let topology = current_topology_public_closeout_alignment_summary().map_err(|error| {
        WorthTouchedGraphConflictPublicCloseoutError::new(
            WorthTouchedGraphConflictPublicCloseoutErrorKind::CurrentProofUnavailable,
            format!(
                "phase 16 topology public closeout support did not assemble: {}",
                error.detail()
            ),
        )
    })?;
    let spatial = current_spatial_public_closeout_alignment_summary().map_err(|error| {
        WorthTouchedGraphConflictPublicCloseoutError::new(
            WorthTouchedGraphConflictPublicCloseoutErrorKind::CurrentProofUnavailable,
            format!(
                "phase 16 spatial public closeout support did not assemble: {}",
                error.detail()
            ),
        )
    })?;
    let deleted_authority_rows = deletion_closeout
        .deletion_ledger()
        .rows()
        .iter()
        .filter(|row| {
            row.disposition() == WorthTouchedGraphConflictDeletionDisposition::DeletedAuthority
        })
        .map(WorthTouchedGraphConflictDeletionAlignmentRow::from_deletion_row)
        .collect::<Vec<_>>();
    let displaced_legacy_authority_rows = deletion_closeout
        .deletion_ledger()
        .rows()
        .iter()
        .map(WorthTouchedGraphConflictDeletionAlignmentRow::from_deletion_row)
        .collect::<Vec<_>>();
    let capped_deletion_rows = deletion_closeout
        .deletion_ledger()
        .rows()
        .iter()
        .filter(|row| {
            row.disposition() == WorthTouchedGraphConflictDeletionDisposition::CappedResidue
        })
        .map(WorthTouchedGraphConflictDeletionAlignmentRow::from_deletion_row)
        .collect::<Vec<_>>();
    let certification_only_fence_rows = deletion_closeout
        .deletion_ledger()
        .rows()
        .iter()
        .filter(|row| {
            row.disposition()
                == WorthTouchedGraphConflictDeletionDisposition::CertificationOnlyFence
        })
        .map(WorthTouchedGraphConflictDeletionAlignmentRow::from_deletion_row)
        .collect::<Vec<_>>();
    let capped_residue_rows = residue_chain
        .rows()
        .iter()
        .filter(|row| {
            row.boundary_posture()
                != WorthTouchedGraphConflictResidueBoundaryPosture::CoveredOrdinaryConsumerDependency
        })
        .map(WorthTouchedGraphConflictArchitectureAlignmentReportRow::from_residue_row)
        .collect::<Vec<_>>();
    let ordinary_second_ontology_blockers = residue_chain
        .rows()
        .iter()
        .filter(|row| {
            row.boundary_posture()
                == WorthTouchedGraphConflictResidueBoundaryPosture::CoveredOrdinaryConsumerDependency
        })
        .map(WorthTouchedGraphConflictArchitectureAlignmentReportRow::from_residue_row)
        .collect::<Vec<_>>();
    let report_digest = truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &deleted_authority_rows
            .iter()
            .map(|row| format!("deleted:{}:{}", row.source_path(), row.surface_name()))
            .chain(capped_deletion_rows.iter().map(|row| {
                format!(
                    "capped-deletion:{}:{}",
                    row.source_path(),
                    row.surface_name()
                )
            }))
            .chain(certification_only_fence_rows.iter().map(|row| {
                format!(
                    "certification-only:{}:{}",
                    row.source_path(),
                    row.surface_name()
                )
            }))
            .chain(
                capped_residue_rows
                    .iter()
                    .map(|row| format!("capped-residue:{}:{}", row.owner(), row.surface_name())),
            )
            .chain(
                ordinary_second_ontology_blockers
                    .iter()
                    .map(|row| format!("second-ontology:{}:{}", row.owner(), row.surface_name())),
            )
            .chain(std::iter::once(format!(
                "topology-compiled-product:{}",
                topology.compiled_product_identity_digest()
            )))
            .chain(std::iter::once(format!(
                "topology-equivalence-policy:{}",
                topology.equivalence_policy_identity_digest()
            )))
            .chain(std::iter::once(format!(
                "spatial-compiled-product:{}",
                spatial.compiled_product_identity_digest()
            )))
            .chain(std::iter::once(format!(
                "spatial-equivalence-policy:{}",
                spatial.equivalence_policy_identity_digest()
            )))
            .chain(std::iter::once(format!(
                "reuse-decision:{}",
                topology
                    .reuse_decision_identity_digest()
                    .unwrap_or("not-applicable")
            )))
            .chain(std::iter::once(format!(
                "rebuild-denial:{}",
                topology
                    .rebuild_denial_identity_digest()
                    .unwrap_or("not-applicable")
            )))
            .chain(std::iter::once(
                "worth-kernel:touched-graph-conflict-architecture-alignment-report:v1".to_string(),
            ))
            .collect::<Vec<_>>(),
    );
    Ok(WorthTouchedGraphConflictArchitectureAlignmentReport {
        topology_compiled_product_identity_digest: topology
            .compiled_product_identity_digest()
            .to_string(),
        topology_equivalence_policy_identity_digest: topology
            .equivalence_policy_identity_digest()
            .to_string(),
        spatial_compiled_product_identity_digest: spatial
            .compiled_product_identity_digest()
            .to_string(),
        spatial_equivalence_policy_identity_digest: spatial
            .equivalence_policy_identity_digest()
            .to_string(),
        reuse_decision_identity_digest: topology
            .reuse_decision_identity_digest()
            .map(str::to_string),
        rebuild_denial_identity_digest: topology
            .rebuild_denial_identity_digest()
            .map(str::to_string),
        displaced_legacy_authority_rows,
        deleted_authority_rows,
        capped_deletion_rows,
        certification_only_fence_rows,
        capped_residue_rows,
        ordinary_second_ontology_blockers,
        report_digest,
    })
}

impl WorthTouchedGraphConflictArchitectureAlignmentReport {
    pub fn topology_compiled_product_identity_digest(&self) -> &str {
        &self.topology_compiled_product_identity_digest
    }

    pub fn topology_equivalence_policy_identity_digest(&self) -> &str {
        &self.topology_equivalence_policy_identity_digest
    }

    pub fn spatial_compiled_product_identity_digest(&self) -> &str {
        &self.spatial_compiled_product_identity_digest
    }

    pub fn spatial_equivalence_policy_identity_digest(&self) -> &str {
        &self.spatial_equivalence_policy_identity_digest
    }

    pub fn reuse_decision_identity_digest(&self) -> Option<&str> {
        self.reuse_decision_identity_digest.as_deref()
    }

    pub fn rebuild_denial_identity_digest(&self) -> Option<&str> {
        self.rebuild_denial_identity_digest.as_deref()
    }

    pub fn deleted_authority_rows(&self) -> &[WorthTouchedGraphConflictDeletionAlignmentRow] {
        &self.deleted_authority_rows
    }

    pub fn capped_deletion_rows(&self) -> &[WorthTouchedGraphConflictDeletionAlignmentRow] {
        &self.capped_deletion_rows
    }

    pub fn certification_only_fence_rows(
        &self,
    ) -> &[WorthTouchedGraphConflictDeletionAlignmentRow] {
        &self.certification_only_fence_rows
    }

    pub fn capped_residue_rows(
        &self,
    ) -> &[WorthTouchedGraphConflictArchitectureAlignmentReportRow] {
        &self.capped_residue_rows
    }

    pub fn ordinary_second_ontology_blockers(
        &self,
    ) -> &[WorthTouchedGraphConflictArchitectureAlignmentReportRow] {
        &self.ordinary_second_ontology_blockers
    }

    pub fn displaced_legacy_authority_rows(
        &self,
    ) -> &[WorthTouchedGraphConflictDeletionAlignmentRow] {
        &self.displaced_legacy_authority_rows
    }

    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }

    pub fn milestone_fifteen_ready(&self) -> bool {
        self.ordinary_second_ontology_blockers.is_empty()
    }
}

impl WorthTouchedGraphConflictArchitectureAlignmentReportRow {
    fn from_residue_row(row: &WorthTouchedGraphConflictResidueRow) -> Self {
        Self {
            surface_name: row.surface_name().to_string(),
            owner: row.owner().to_string(),
            blocker: row.blocker().to_string(),
            removal_trigger: row.removal_trigger().to_string(),
        }
    }

    pub fn surface_name(&self) -> &str {
        &self.surface_name
    }

    pub fn owner(&self) -> &str {
        &self.owner
    }

    pub fn blocker(&self) -> &str {
        &self.blocker
    }

    pub fn removal_trigger(&self) -> &str {
        &self.removal_trigger
    }
}

impl WorthTouchedGraphConflictDeletionAlignmentRow {
    fn from_deletion_row(row: &WorthTouchedGraphConflictDeletionLedgerRow) -> Self {
        Self {
            source_path: row.source_path().to_string(),
            surface_name: row.surface_name().to_string(),
            blocker: row.blocker().to_string(),
            removal_trigger: row.removal_trigger().to_string(),
        }
    }

    pub fn source_path(&self) -> &str {
        &self.source_path
    }

    pub fn surface_name(&self) -> &str {
        &self.surface_name
    }

    pub fn blocker(&self) -> &str {
        &self.blocker
    }

    pub fn removal_trigger(&self) -> &str {
        &self.removal_trigger
    }
}
