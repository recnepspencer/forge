use super::super::admission::{
    plan_read_compatibility, CompatibilityAdmissionBatch, CompatibilityEdgeRegistry,
    CompatibilityReadAdmissionOutcome, CompatibilityReadIntent, CompatibilityRejectionKind,
    CompatibilityRelation, DeclaredCompatibilityEdge, ReaderCapabilitySet,
};

use super::super::decoding::QuarantinedDecodedArtifact;
use worth_store_contracts::CompatibilityFamilyKind;

use super::super::certification::{
    Milestone12CertificationLaneKind, Milestone12CertificationLaneOutcome,
    Milestone12CertificationLaneRejection,
};

use super::super::manifests::ArtifactSemanticVersion;

use super::scenario_inputs::{artifact_for_family, lane_input};

pub(super) fn authoritative_lanes(
    manifest_index: &super::super::admission::CompatibilityManifestIndex,
) -> Result<Vec<Milestone12CertificationLaneOutcome>, Milestone12CertificationLaneRejection> {
    let family_id = CompatibilityFamilyKind::CommitEnvelope.family_id();
    let artifact = artifact_for_family(CompatibilityFamilyKind::CommitEnvelope, 1);
    let lanes = [
        (
            Milestone12CertificationLaneKind::AuthoritativeNativeRead,
            CompatibilityRelation::Native,
            Some(CompatibilityRelation::Native),
            None,
            1,
        ),
        (
            Milestone12CertificationLaneKind::AuthoritativeForwardRead,
            CompatibilityRelation::ForwardRead,
            Some(CompatibilityRelation::ForwardRead),
            None,
            2,
        ),
        (
            Milestone12CertificationLaneKind::AuthoritativeBackwardRead,
            CompatibilityRelation::BackwardRead,
            Some(CompatibilityRelation::BackwardRead),
            None,
            2,
        ),
    ];
    let mut outcomes = Vec::new();
    for (kind, relation, expected_relation, expected_rejection, target) in lanes {
        let edge_registry = CompatibilityEdgeRegistry::new(vec![DeclaredCompatibilityEdge::new(
            family_id.clone(),
            ArtifactSemanticVersion::new(1),
            ArtifactSemanticVersion::new(target),
            relation,
        )]);
        outcomes.push(read_lane(
            kind,
            manifest_index,
            &edge_registry,
            &artifact,
            target,
            expected_relation,
            expected_rejection,
        )?);
    }

    outcomes.push(read_lane(
        Milestone12CertificationLaneKind::AuthoritativeMissingEdgeRejected,
        manifest_index,
        &CompatibilityEdgeRegistry::new(Vec::new()),
        &artifact,
        2,
        None,
        Some(CompatibilityRejectionKind::MissingCompatibilityEdge),
    )?);
    let incompatible_edges = CompatibilityEdgeRegistry::new(vec![DeclaredCompatibilityEdge::new(
        family_id,
        ArtifactSemanticVersion::new(1),
        ArtifactSemanticVersion::new(1),
        CompatibilityRelation::Incompatible,
    )]);
    outcomes.push(read_lane(
        Milestone12CertificationLaneKind::AuthoritativeIncompatibleEdgeRejected,
        manifest_index,
        &incompatible_edges,
        &artifact,
        1,
        None,
        Some(CompatibilityRejectionKind::DeclaredIncompatibleRelation),
    )?);
    Ok(outcomes)
}

fn read_lane(
    kind: Milestone12CertificationLaneKind,
    manifest_index: &super::super::admission::CompatibilityManifestIndex,
    edge_registry: &CompatibilityEdgeRegistry,
    artifact: &QuarantinedDecodedArtifact,
    target_semantic_version: u32,
    expected_relation: Option<CompatibilityRelation>,
    expected_rejection: Option<CompatibilityRejectionKind>,
) -> Result<Milestone12CertificationLaneOutcome, Milestone12CertificationLaneRejection> {
    let mut batch = CompatibilityAdmissionBatch::new();
    let family_id = artifact.family_id().clone();
    let reader = ReaderCapabilitySet::new(
        family_id.clone(),
        vec![ArtifactSemanticVersion::new(target_semantic_version)],
    );
    let intent = CompatibilityReadIntent::new(
        family_id.clone(),
        ArtifactSemanticVersion::new(target_semantic_version),
    );
    let outcome = match plan_read_compatibility(
        &mut batch,
        manifest_index,
        edge_registry,
        &reader,
        &intent,
        artifact,
    ) {
        Ok(receipt) => CompatibilityReadAdmissionOutcome::accepted(&receipt, batch.counters()),
        Err(rejection) => {
            CompatibilityReadAdmissionOutcome::rejected(artifact, &rejection, batch.counters())
        }
    };
    Milestone12CertificationLaneOutcome::from_read_outcome(
        kind,
        lane_input(
            family_id,
            artifact.semantic_version().value(),
            target_semantic_version,
            expected_relation,
            expected_rejection,
        ),
        &outcome,
    )
}
