use std::collections::BTreeSet;

use worth_primitives::{truth_digest_parts, TruthDigestScope};

use crate::workload_platform::evidence_ledger::SpatialEvidenceSurfaceCloseoutPosture;
use crate::workload_platform::evidence_lookup_family_catalog::{
    current_evidence_lookup_family_catalog, EvidenceLookupFamilyDeclaration,
};
use crate::workload_platform::evidence_lookup_query_surface_matrix::EvidenceLookupQuerySurfaceTouchpoint;

use super::assembly_input::EvidenceLookupPublicCloseoutAssemblyInput;
use super::closeout_artifacts::{
    EvidenceLookupPublicCloseout, EvidenceLookupPublicCloseoutDisposition,
};
use super::counters::EvidenceLookupPublicCloseoutCounters;
use super::error::{EvidenceLookupPublicCloseoutError, EvidenceLookupPublicCloseoutErrorKind};
use super::milestone_twelve_seed_lowering::lower_milestone_twelve_seed;

impl EvidenceLookupPublicCloseout {
    pub fn assemble_from_proof_products(
        input: &EvidenceLookupPublicCloseoutAssemblyInput,
    ) -> Result<Self, EvidenceLookupPublicCloseoutError> {
        let family_catalog = current_evidence_lookup_family_catalog().map_err(|error| {
            EvidenceLookupPublicCloseoutError::new(
                EvidenceLookupPublicCloseoutErrorKind::MissingFamilyCoverageDisposition,
                format!(
                    "family catalog failed during closeout assembly: {:?}",
                    error.kind()
                ),
            )
        })?;

        reject_duplicate_family_stage_rows(input)?;
        reject_mismatched_authority_chain(input, family_catalog.declarations())?;
        reject_firewall_without_deletion_pressure(input)?;

        let counters = EvidenceLookupPublicCloseoutCounters::new(
            input.family_stage_rows().len(),
            input
                .family_stage_rows()
                .iter()
                .filter(|row| {
                    matches!(
                        row.disposition(),
                        EvidenceLookupPublicCloseoutDisposition::ReceiptProof { .. }
                    )
                })
                .count(),
            input
                .family_stage_rows()
                .iter()
                .filter(|row| {
                    matches!(
                        row.disposition(),
                        EvidenceLookupPublicCloseoutDisposition::NonOrdinaryResidue { .. }
                    )
                })
                .count(),
            input.query_surface_matrix().rows().len(),
            input.query_consumer_kit().binding_rows().len(),
            input.query_consumer_kit().support_rows().len(),
            input.spatial_deletion_ledger_rows().len(),
            input
                .spatial_deletion_ledger_rows()
                .iter()
                .filter(|row| {
                    matches!(
                        row.closeout_posture(),
                        SpatialEvidenceSurfaceCloseoutPosture::CertificationOnly
                            | SpatialEvidenceSurfaceCloseoutPosture::CappedResidue
                    )
                })
                .count(),
            input.query_consumer_kit().query_residue_rows().len(),
            input
                .source_firewall_report()
                .counters()
                .forbidden_row_count(),
            input
                .source_firewall_report()
                .counters()
                .allowed_exception_row_count(),
        );
        let family_coverage_digest = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &input
                .family_stage_rows()
                .iter()
                .map(|row| format!("family-row:{}", row.row_digest()))
                .collect::<Vec<_>>(),
        );
        let spatial_deletion_ledger_digest = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &input
                .spatial_deletion_ledger_rows()
                .iter()
                .map(|row| {
                    format!(
                        "deletion:{}:{}:{:?}",
                        row.surface_name(),
                        row.source_path(),
                        row.closeout_posture()
                    )
                })
                .collect::<Vec<_>>(),
        );
        let residue_audit_digest = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &input
                .family_stage_rows()
                .iter()
                .filter_map(|row| match row.disposition() {
                    EvidenceLookupPublicCloseoutDisposition::NonOrdinaryResidue { .. } => {
                        Some(format!("family-residue:{}", row.row_digest()))
                    }
                    EvidenceLookupPublicCloseoutDisposition::ReceiptProof { .. } => None,
                })
                .chain(
                    input
                        .query_consumer_kit()
                        .query_residue_rows()
                        .iter()
                        .map(|row| format!("query-residue:{}", row.row_digest())),
                )
                .chain(
                    input
                        .spatial_deletion_ledger_rows()
                        .iter()
                        .filter_map(|row| {
                            matches!(
                                row.closeout_posture(),
                                SpatialEvidenceSurfaceCloseoutPosture::CertificationOnly
                                    | SpatialEvidenceSurfaceCloseoutPosture::CappedResidue
                            )
                            .then(|| {
                                format!(
                                    "spatial-residue:{}:{}",
                                    row.surface_name(),
                                    row.source_path()
                                )
                            })
                        }),
                )
                .collect::<Vec<_>>(),
        );
        let closeout_digest = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &[
                "worth-spatial:evidence-lookup-public-closeout:v1".to_string(),
                format!("family-coverage:{family_coverage_digest}"),
                format!(
                    "query-matrix:{}",
                    input.query_surface_matrix().matrix_digest()
                ),
                format!(
                    "consumer-kit:{}",
                    input.query_consumer_kit().closeout_digest()
                ),
                format!(
                    "source-firewall:{}",
                    input.source_firewall_report().firewall_digest()
                ),
                format!("deletion-ledger:{spatial_deletion_ledger_digest}"),
                format!("residue-audit:{residue_audit_digest}"),
                format!("family-stage-rows:{}", counters.family_stage_row_count()),
                format!("receipt-proof-rows:{}", counters.receipt_proof_row_count()),
                format!(
                    "non-ordinary-residue-rows:{}",
                    counters.non_ordinary_residue_row_count()
                ),
            ],
        );
        let milestone_twelve_seed = lower_milestone_twelve_seed(
            &closeout_digest,
            input.query_surface_matrix().matrix_digest(),
            input.query_consumer_kit().closeout_digest(),
            input.source_firewall_report().firewall_digest(),
            &residue_audit_digest,
            &family_coverage_digest,
            input.family_stage_rows(),
            &counters,
        );

        Ok(Self {
            family_stage_rows: input.family_stage_rows().to_vec(),
            query_surface_matrix: input.query_surface_matrix().clone(),
            query_consumer_kit: input.query_consumer_kit().clone(),
            source_firewall_report: input.source_firewall_report().clone(),
            spatial_deletion_ledger_rows: input.spatial_deletion_ledger_rows().to_vec(),
            counters,
            family_coverage_digest,
            spatial_deletion_ledger_digest,
            residue_audit_digest,
            milestone_twelve_seed,
            closeout_digest,
        })
    }
}

fn reject_duplicate_family_stage_rows(
    input: &EvidenceLookupPublicCloseoutAssemblyInput,
) -> Result<(), EvidenceLookupPublicCloseoutError> {
    let mut identities = BTreeSet::new();
    for row in input.family_stage_rows() {
        let identity = format!("{}::{:?}", row.family_identity(), row.stage());
        if !identities.insert(identity) {
            return Err(EvidenceLookupPublicCloseoutError::new(
                EvidenceLookupPublicCloseoutErrorKind::DuplicateFamilyStageRow,
                format!(
                    "duplicate public closeout family-stage row for `{}` at stage `{:?}`",
                    row.family_identity(),
                    row.stage()
                ),
            ));
        }
    }
    Ok(())
}

fn reject_mismatched_authority_chain(
    input: &EvidenceLookupPublicCloseoutAssemblyInput,
    families: &[EvidenceLookupFamilyDeclaration],
) -> Result<(), EvidenceLookupPublicCloseoutError> {
    for row in input.family_stage_rows() {
        let query_row = input
            .query_surface_matrix()
            .require_family_stage_touchpoint_row(
                row.family_identity(),
                row.stage(),
                EvidenceLookupQuerySurfaceTouchpoint::PublicCloseoutProof,
            )
            .map_err(|_| {
                EvidenceLookupPublicCloseoutError::new(
                    EvidenceLookupPublicCloseoutErrorKind::MissingPublicCloseoutQueryRow,
                    format!(
                        "missing public-closeout query matrix row for family `{}` at stage `{}`",
                        row.family_identity(),
                        row.stage().human_name()
                    ),
                )
            })?;
        if query_row.row_digest() != row.query_surface_row_digest() {
            return Err(EvidenceLookupPublicCloseoutError::new(
                EvidenceLookupPublicCloseoutErrorKind::MismatchedFamilyAuthorityChain,
                format!(
                    "family `{}` at stage `{}` carries a mismatched public-closeout query row digest",
                    row.family_identity(),
                    row.stage().human_name()
                ),
            ));
        }

        let family = families
            .iter()
            .find(|family| family.identity().as_str() == row.family_identity())
            .ok_or_else(|| {
                EvidenceLookupPublicCloseoutError::new(
                    EvidenceLookupPublicCloseoutErrorKind::MissingFamilyCoverageDisposition,
                    format!(
                        "closeout row references undeclared family `{}`",
                        row.family_identity()
                    ),
                )
            })?;
        if family.declaration_digest() != row.family_declaration_digest()
            || family
                .stage_applicability()
                .stage_receipt_family_identity()
                .digest()
                != row.stage_receipt_family_identity()
            || family.query_posture().imported_evidence_digest()
                != row.query_import_evidence_digest()
        {
            return Err(EvidenceLookupPublicCloseoutError::new(
                EvidenceLookupPublicCloseoutErrorKind::MismatchedFamilyAuthorityChain,
                format!(
                    "family `{}` at stage `{}` is not bound to the current catalog authority chain",
                    row.family_identity(),
                    row.stage().human_name()
                ),
            ));
        }
        let topology_requires_receipt = family.topology_input_posture().requires_topology_receipt();
        match row.disposition() {
            EvidenceLookupPublicCloseoutDisposition::ReceiptProof { .. } => {
                if topology_requires_receipt || row.spatial_touch_digest().is_none() {
                    return Err(EvidenceLookupPublicCloseoutError::new(
                        EvidenceLookupPublicCloseoutErrorKind::ImpossibleResidueSuccessMix,
                        format!(
                            "family `{}` at stage `{}` cannot publish receipt proof with the current topology/spatial-touch posture",
                            row.family_identity(),
                            row.stage().human_name()
                        ),
                    ));
                }
            }
            EvidenceLookupPublicCloseoutDisposition::NonOrdinaryResidue { .. } => {
                if !topology_requires_receipt {
                    return Err(EvidenceLookupPublicCloseoutError::new(
                        EvidenceLookupPublicCloseoutErrorKind::ImpossibleResidueSuccessMix,
                        format!(
                            "family `{}` at stage `{}` cannot publish residue without a topology-backed blocker",
                            row.family_identity(),
                            row.stage().human_name()
                        ),
                    ));
                }
            }
        }
    }
    Ok(())
}

fn reject_firewall_without_deletion_pressure(
    input: &EvidenceLookupPublicCloseoutAssemblyInput,
) -> Result<(), EvidenceLookupPublicCloseoutError> {
    if input
        .source_firewall_report()
        .counters()
        .forbidden_row_count()
        > 0
        && input.spatial_deletion_ledger_rows().is_empty()
    {
        return Err(EvidenceLookupPublicCloseoutError::new(
            EvidenceLookupPublicCloseoutErrorKind::SourceFirewallDeletionPressureMismatch,
            "forbidden source-firewall authority requires concrete deletion-ledger pressure",
        ));
    }
    Ok(())
}
