use super::super::adapters::{
    execute_declared_adapter_parity, first_ship_authoritative_adapter_edge_registry,
};

use super::super::admission::{
    check_artifact_with_read_receipt, plan_read_compatibility_for_path, CompatibilityAdapterDigest,
    CompatibilityAdapterId, CompatibilityAdmissionBatch, CompatibilityAdmissionPath,
    CompatibilityEdgeRegistry, CompatibilityReadIntent, CompatibilityRejectionKind,
    CompatibilityRelation, ReaderCapabilitySet,
};

use super::super::authoritative::{
    admit_authoritative_meaning_with_parity_witness, declare_authoritative_meaning,
};

use worth_store_contracts::CompatibilityFamilyKind;

use super::super::certification::{
    Milestone12CertificationLaneKind, Milestone12CertificationLaneOutcome,
};

use super::super::manifests::{ArtifactFamilyId, ArtifactSemanticVersion};

use super::scenario_inputs::{artifact_for_family, lane_input};

pub(super) fn adapter_lanes(
    manifest_index: &super::super::admission::CompatibilityManifestIndex,
) -> Vec<Milestone12CertificationLaneOutcome> {
    let family_kind = CompatibilityFamilyKind::CommitEnvelope;
    let edge_registry = first_ship_authoritative_adapter_edge_registry();
    let payload = b"first-ship-certification-adapter-control".to_vec();
    vec![
        adapter_lane(
            Milestone12CertificationLaneKind::AdapterParityAdmitted,
            manifest_index,
            &edge_registry,
            family_kind,
            CompatibilityAdapterDigest::new("first_ship_commit_envelope_adapter_digest_v1"),
            payload.clone(),
            payload.clone(),
            Some(CompatibilityRelation::AdapterRequired),
            None,
        ),
        adapter_lane(
            Milestone12CertificationLaneKind::AdapterParityDigestRejected,
            manifest_index,
            &edge_registry,
            family_kind,
            CompatibilityAdapterDigest::new("drifted-adapter-digest"),
            payload,
            b"first-ship-certification-adapter-adapted".to_vec(),
            None,
            Some(CompatibilityRejectionKind::AdapterParityFailure),
        ),
    ]
}

fn adapter_lane(
    lane_kind: Milestone12CertificationLaneKind,
    manifest_index: &super::super::admission::CompatibilityManifestIndex,
    edge_registry: &CompatibilityEdgeRegistry,
    family_kind: CompatibilityFamilyKind,
    requested_digest: CompatibilityAdapterDigest,
    control_lane_bytes: Vec<u8>,
    adapted_lane_bytes: Vec<u8>,
    expected_relation: Option<CompatibilityRelation>,
    expected_rejection: Option<CompatibilityRejectionKind>,
) -> Milestone12CertificationLaneOutcome {
    let artifact = artifact_for_family(family_kind, 1);
    let family_id = family_id_for_lane(family_kind);
    let mut batch = CompatibilityAdmissionBatch::new();
    let input = lane_input(
        family_id.clone(),
        1,
        2,
        expected_relation,
        expected_rejection,
    );
    let reader = ReaderCapabilitySet::new(family_id.clone(), vec![ArtifactSemanticVersion::new(2)]);
    let intent = CompatibilityReadIntent::new(family_id.clone(), ArtifactSemanticVersion::new(2));
    let read_receipt = match plan_read_compatibility_for_path(
        &mut batch,
        manifest_index,
        edge_registry,
        &reader,
        &intent,
        &artifact,
        CompatibilityAdmissionPath::BatchRead,
    ) {
        Ok(receipt) => receipt,
        Err(rejection) => {
            return Milestone12CertificationLaneOutcome::from_compatibility_rejection(
                lane_kind,
                input,
                &rejection,
                batch.counters(),
            )
        }
    };
    let checked_artifact = match check_artifact_with_read_receipt(artifact, &read_receipt) {
        Ok(artifact) => artifact,
        Err(rejection) => {
            return Milestone12CertificationLaneOutcome::from_compatibility_rejection(
                lane_kind,
                input,
                &rejection,
                batch.counters(),
            )
        }
    };
    let parity = match execute_declared_adapter_parity(
        batch.counters_mut(),
        edge_registry,
        &family_id,
        ArtifactSemanticVersion::new(1),
        ArtifactSemanticVersion::new(2),
        &CompatibilityAdapterId::new("first_ship_commit_envelope_adapter"),
        &requested_digest,
        &control_lane_bytes,
        &adapted_lane_bytes,
        1,
        1,
        1,
    ) {
        Ok(parity) => parity,
        Err(rejection) => {
            return Milestone12CertificationLaneOutcome::from_compatibility_rejection(
                lane_kind,
                input,
                &rejection,
                batch.counters(),
            )
        }
    };
    let meaning = declare_authoritative_meaning(
        family_id,
        ArtifactSemanticVersion::new(2),
        "first-ship-certification-adapter-meaning",
    );
    if let Err(rejection) = admit_authoritative_meaning_with_parity_witness(
        batch.counters_mut(),
        &checked_artifact,
        &read_receipt,
        Some(&meaning),
        Some(&parity),
    ) {
        return Milestone12CertificationLaneOutcome::from_compatibility_rejection(
            lane_kind,
            input,
            &rejection,
            batch.counters(),
        );
    }
    Milestone12CertificationLaneOutcome::accepted(
        lane_kind,
        input,
        read_receipt.receipt().relation(),
        batch.counters(),
    )
}

fn family_id_for_lane(family_kind: CompatibilityFamilyKind) -> ArtifactFamilyId {
    family_kind.family_id()
}
