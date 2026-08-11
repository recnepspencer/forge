use super::super::admission::{
    plan_read_compatibility, CompatibilityAdmissionBatch, CompatibilityEdgeRegistry,
    CompatibilityReadIntent, CompatibilityRejectionKind, CompatibilityRelation,
    DeclaredCompatibilityEdge, ReaderCapabilitySet,
};

use super::super::catalog::{CompatibilityRegistrySnapshot, DerivedFamilyDeclaration};
use worth_store_contracts::CompatibilityFamilyKind;

use super::super::certification::{
    Milestone12CertificationLaneKind, Milestone12CertificationLaneOutcome,
    Milestone12CertificationLaneRejection,
};

use super::super::derived::{
    plan_derived_lane_compatibility, DerivedBasisCompatibilityInput,
    DerivedCompatibilityLaneRegistry,
};

use super::super::manifests::{ArtifactCompatibilityWindow, ArtifactSemanticVersion};

use crate::{CompatibilityDerivedRebuildRequest, WORTHStoreBuilder};

use super::scenario_inputs::{artifact_for_family, lane_input};

pub(super) fn derived_lanes(
    snapshot: &CompatibilityRegistrySnapshot,
    manifest_index: &super::super::admission::CompatibilityManifestIndex,
) -> Result<Vec<Milestone12CertificationLaneOutcome>, Milestone12CertificationLaneRejection> {
    let lanes = DerivedCompatibilityLaneRegistry::from_compatibility_snapshot(snapshot).snapshot();
    let mut outcomes = Vec::new();
    outcomes.push(derived_lane(
        snapshot,
        &lanes,
        manifest_index,
        CompatibilityFamilyKind::SnapshotRecord,
        Milestone12CertificationLaneKind::DerivedSnapshotReuseAccepted,
        CompatibilityRelation::Native,
        1,
        ArtifactCompatibilityWindow::native(1),
        Some(CompatibilityRelation::Native),
        None,
    )?);
    outcomes.push(derived_rebuild_execution_lane()?);
    outcomes.push(derived_lane(
        snapshot,
        &lanes,
        manifest_index,
        CompatibilityFamilyKind::Milestone6LayoutBlockChunkRecord,
        Milestone12CertificationLaneKind::DerivedLayoutBasisRejected,
        CompatibilityRelation::Native,
        1,
        ArtifactCompatibilityWindow::native(2),
        None,
        Some(CompatibilityRejectionKind::DerivedBasisIncompatible),
    )?);
    outcomes.push(derived_lane(
        snapshot,
        &lanes,
        manifest_index,
        CompatibilityFamilyKind::Milestone9BulkRecord,
        Milestone12CertificationLaneKind::DerivedBulkResumeRejected,
        CompatibilityRelation::ForwardRead,
        2,
        ArtifactCompatibilityWindow::native(1),
        None,
        Some(CompatibilityRejectionKind::BulkResumeCompatibilityRejected),
    )?);
    outcomes.push(derived_lane(
        snapshot,
        &lanes,
        manifest_index,
        CompatibilityFamilyKind::Milestone13TieringRecord,
        Milestone12CertificationLaneKind::TierManifestNonAuthorityPreserved,
        CompatibilityRelation::Native,
        1,
        ArtifactCompatibilityWindow::native(1),
        Some(CompatibilityRelation::Native),
        None,
    )?);
    Ok(outcomes)
}

fn derived_rebuild_execution_lane(
) -> Result<Milestone12CertificationLaneOutcome, Milestone12CertificationLaneRejection> {
    let mut store = WORTHStoreBuilder::new()
        .in_memory()
        .build()
        .expect("first-ship certification rebuild store should build");
    let outcome = store
        .execute_compatibility_derived_rebuild(CompatibilityDerivedRebuildRequest::new(
            CompatibilityFamilyKind::Milestone11MaintenanceRecord,
        ))
        .expect("first-ship certification rebuild lane should execute");
    Ok(Milestone12CertificationLaneOutcome::accepted_from_report(
        Milestone12CertificationLaneKind::MaintenanceSummaryRebuildAdmitted,
        lane_input(
            CompatibilityFamilyKind::Milestone11MaintenanceRecord.family_id(),
            1,
            2,
            Some(CompatibilityRelation::Native),
            None,
        ),
        CompatibilityRelation::Native,
        outcome.admission_report().clone(),
    ))
}

fn derived_lane(
    snapshot: &CompatibilityRegistrySnapshot,
    lanes: &super::super::derived::DerivedCompatibilityLaneSnapshot,
    manifest_index: &super::super::admission::CompatibilityManifestIndex,
    family_kind: CompatibilityFamilyKind,
    lane_kind: Milestone12CertificationLaneKind,
    relation: CompatibilityRelation,
    target_semantic_version: u32,
    required_window: ArtifactCompatibilityWindow,
    expected_relation: Option<CompatibilityRelation>,
    expected_rejection: Option<CompatibilityRejectionKind>,
) -> Result<Milestone12CertificationLaneOutcome, Milestone12CertificationLaneRejection> {
    let family_id = family_kind.family_id();
    let artifact = artifact_for_family(family_kind, 1);
    let edge_registry = CompatibilityEdgeRegistry::new(vec![DeclaredCompatibilityEdge::new(
        family_id.clone(),
        ArtifactSemanticVersion::new(1),
        ArtifactSemanticVersion::new(target_semantic_version),
        relation,
    )]);
    let mut batch = CompatibilityAdmissionBatch::new();
    let reader = ReaderCapabilitySet::new(
        family_id.clone(),
        vec![ArtifactSemanticVersion::new(target_semantic_version)],
    );
    let intent = CompatibilityReadIntent::new(
        family_id.clone(),
        ArtifactSemanticVersion::new(target_semantic_version),
    );
    let receipt = match plan_read_compatibility(
        &mut batch,
        manifest_index,
        &edge_registry,
        &reader,
        &intent,
        &artifact,
    ) {
        Ok(receipt) => receipt,
        Err(rejection) => {
            return Ok(
                Milestone12CertificationLaneOutcome::from_compatibility_rejection(
                    lane_kind,
                    lane_input(
                        family_id,
                        1,
                        target_semantic_version,
                        expected_relation,
                        expected_rejection,
                    ),
                    &rejection,
                    batch.counters(),
                ),
            );
        }
    };
    let lane_declaration = lanes
        .get_by_family_kind(family_kind)
        .expect("first-ship derived lane exists")
        .clone();
    let derived_family = DerivedFamilyDeclaration::new(
        snapshot
            .get(family_kind)
            .expect("first-ship derived family exists")
            .clone(),
    );
    let input =
        DerivedBasisCompatibilityInput::new(lane_declaration, derived_family, required_window);
    let plan = plan_derived_lane_compatibility(batch.counters_mut(), &input, &artifact, &receipt);
    let certification_input = lane_input(
        family_id,
        artifact.semantic_version().value(),
        target_semantic_version,
        expected_relation,
        expected_rejection,
    );
    Ok(match plan {
        Ok(plan) => Milestone12CertificationLaneOutcome::from_derived_plan(
            lane_kind,
            certification_input,
            &plan,
            batch.counters(),
        ),
        Err(rejection) => Milestone12CertificationLaneOutcome::from_compatibility_rejection(
            lane_kind,
            certification_input,
            &rejection,
            batch.counters(),
        ),
    })
}
