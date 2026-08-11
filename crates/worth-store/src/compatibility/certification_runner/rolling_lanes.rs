use super::super::admission::{
    CompatibilityAdapterCostClass, CompatibilityAdmissionCounters, CompatibilityEdgeRegistry,
    CompatibilityRejectionKind, CompatibilityRelation, DeclaredCompatibilityEdge,
    ReaderCapabilitySet, WriterCapabilitySet,
};

use worth_store_contracts::CompatibilityFamilyKind;

use super::super::certification::{
    Milestone12CertificationLaneKind, Milestone12CertificationLaneOutcome,
};

use super::super::manifests::ArtifactSemanticVersion;

use super::super::rolling::{plan_first_ship_rolling_upgrade, RollingUpgradeWindow};

use super::scenario_inputs::{adapter, lane_input, rolling_window};

pub(super) fn rolling_lanes() -> Vec<Milestone12CertificationLaneOutcome> {
    let family_id = CompatibilityFamilyKind::CommitEnvelope.family_id();
    let window = rolling_window(family_id.clone());
    let reader = ReaderCapabilitySet::new(family_id.clone(), vec![ArtifactSemanticVersion::new(1)]);
    let writer = WriterCapabilitySet::new(family_id.clone(), vec![ArtifactSemanticVersion::new(2)]);
    vec![
        rolling_lane(
            Milestone12CertificationLaneKind::RollingTwoCapabilityAdmitted,
            &window,
            &[reader.clone()],
            &[writer.clone()],
            CompatibilityEdgeRegistry::new(vec![DeclaredCompatibilityEdge::new(
                family_id.clone(),
                ArtifactSemanticVersion::new(1),
                ArtifactSemanticVersion::new(2),
                CompatibilityRelation::ForwardRead,
            )]),
            Some(CompatibilityRelation::ForwardRead),
            None,
        ),
        rolling_lane(
            Milestone12CertificationLaneKind::RollingMultiWriterRejected,
            &window,
            &[reader.clone()],
            &[writer.clone(), writer.clone()],
            CompatibilityEdgeRegistry::new(vec![DeclaredCompatibilityEdge::new(
                family_id.clone(),
                ArtifactSemanticVersion::new(1),
                ArtifactSemanticVersion::new(2),
                CompatibilityRelation::ForwardRead,
            )]),
            None,
            Some(CompatibilityRejectionKind::RollingMultiWriterRejected),
        ),
        rolling_lane(
            Milestone12CertificationLaneKind::RollingMissingEdgeRejected,
            &window,
            &[reader.clone()],
            &[writer.clone()],
            CompatibilityEdgeRegistry::new(Vec::new()),
            None,
            Some(CompatibilityRejectionKind::MissingCompatibilityEdge),
        ),
        rolling_lane(
            Milestone12CertificationLaneKind::RollingAdapterEdgeRejected,
            &window,
            &[reader],
            &[writer],
            CompatibilityEdgeRegistry::new(vec![DeclaredCompatibilityEdge::new(
                family_id.clone(),
                ArtifactSemanticVersion::new(1),
                ArtifactSemanticVersion::new(2),
                CompatibilityRelation::AdapterRequired,
            )
            .with_adapter(adapter(CompatibilityAdapterCostClass::BoundedBatchLocal))]),
            None,
            Some(CompatibilityRejectionKind::RollingWindowRejected),
        ),
    ]
}

fn rolling_lane(
    lane_kind: Milestone12CertificationLaneKind,
    window: &RollingUpgradeWindow,
    readers: &[ReaderCapabilitySet],
    writers: &[WriterCapabilitySet],
    edge_registry: CompatibilityEdgeRegistry,
    expected_relation: Option<CompatibilityRelation>,
    expected_rejection: Option<CompatibilityRejectionKind>,
) -> Milestone12CertificationLaneOutcome {
    let mut counters = CompatibilityAdmissionCounters::default();
    let input = lane_input(
        window.family_id().clone(),
        1,
        2,
        expected_relation,
        expected_rejection,
    );
    match plan_first_ship_rolling_upgrade(&mut counters, &edge_registry, window, readers, writers) {
        Ok(plan) => Milestone12CertificationLaneOutcome::from_rolling_plan(input, &plan, &counters),
        Err(rejection) => Milestone12CertificationLaneOutcome::from_compatibility_rejection(
            lane_kind, input, &rejection, &counters,
        ),
    }
}
