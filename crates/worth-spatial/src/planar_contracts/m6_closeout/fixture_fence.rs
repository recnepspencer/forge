use crate::planar_contracts::contract_bundle::planar_contract_bundle_digest;
use crate::workload_platform::inventory::{
    InventoryDecision, LegacyFixtureClassification, ReceiptPosture, SurfaceAuthority, SurfaceKind,
    SurfaceScope, TopologyPosture,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum M6LegacyFixtureFencePosture {
    UnitOnly,
    WorkloadPlatformRecipe,
    SyntheticEndToEndBlocked,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct M6LegacyFixtureFenceRow {
    classification: LegacyFixtureClassification,
    decision: InventoryDecision,
    posture: M6LegacyFixtureFencePosture,
}

impl M6LegacyFixtureFenceRow {
    pub fn classify(
        classification: LegacyFixtureClassification,
        decision: InventoryDecision,
    ) -> Self {
        let posture = classify_fixture_posture(classification, decision);
        Self {
            classification,
            decision,
            posture,
        }
    }

    pub fn classification(&self) -> LegacyFixtureClassification {
        self.classification
    }

    pub fn decision(&self) -> InventoryDecision {
        self.decision
    }

    pub fn posture(&self) -> M6LegacyFixtureFencePosture {
        self.posture
    }

    pub fn fence_digest(&self) -> String {
        planar_contract_bundle_digest(&[
            format!("surface:{}", self.classification.surface_id().as_str()),
            format!("posture:{:?}", self.posture),
            format!("decision:{:?}", self.decision),
        ])
    }

    pub fn human_reason(&self) -> &'static str {
        match self.posture {
            M6LegacyFixtureFencePosture::UnitOnly => {
                "legacy fixture is admitted only as narrow unit support"
            }
            M6LegacyFixtureFencePosture::WorkloadPlatformRecipe => {
                "surface is admitted as a workload-platform recipe"
            }
            M6LegacyFixtureFencePosture::SyntheticEndToEndBlocked => {
                "legacy fixture bypasses production evidence and cannot claim MB closeout authority"
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct M6LegacyFixtureFence {
    rows: Vec<M6LegacyFixtureFenceRow>,
}

impl M6LegacyFixtureFence {
    pub fn from_rows(rows: impl IntoIterator<Item = M6LegacyFixtureFenceRow>) -> Self {
        Self {
            rows: rows.into_iter().collect(),
        }
    }

    pub fn rows(&self) -> &[M6LegacyFixtureFenceRow] {
        &self.rows
    }

    pub fn blocked_synthetic_claims(&self) -> usize {
        self.rows
            .iter()
            .filter(|row| row.posture() == M6LegacyFixtureFencePosture::SyntheticEndToEndBlocked)
            .count()
    }

    pub fn unit_only_fixtures(&self) -> usize {
        self.rows
            .iter()
            .filter(|row| row.posture() == M6LegacyFixtureFencePosture::UnitOnly)
            .count()
    }

    pub fn workload_platform_recipes(&self) -> usize {
        self.rows
            .iter()
            .filter(|row| row.posture() == M6LegacyFixtureFencePosture::WorkloadPlatformRecipe)
            .count()
    }

    pub fn fence_digest(&self) -> String {
        let mut parts: Vec<_> = self
            .rows
            .iter()
            .map(M6LegacyFixtureFenceRow::fence_digest)
            .collect();
        parts.sort();
        planar_contract_bundle_digest(&parts)
    }
}

fn classify_fixture_posture(
    classification: LegacyFixtureClassification,
    decision: InventoryDecision,
) -> M6LegacyFixtureFencePosture {
    if classification.scope() == SurfaceScope::WorkloadCandidate
        && classification.receipt_posture() == ReceiptPosture::ProductionOwned
        && matches!(
            classification.authority(),
            SurfaceAuthority::QueryBackedTopology | SurfaceAuthority::QueryBackedSpatialContract
        )
        && decision == InventoryDecision::ElevateToWorkloadPlatform
    {
        return M6LegacyFixtureFencePosture::WorkloadPlatformRecipe;
    }

    if classification.scope() == SurfaceScope::UnitSupportOnly
        && decision != InventoryDecision::ElevateToWorkloadPlatform
    {
        return M6LegacyFixtureFencePosture::UnitOnly;
    }

    if classification.topology_posture() == TopologyPosture::BypassesTopologyTruth
        || classification.surface_kind() == SurfaceKind::ReExtractionReplayHelper
        || classification.surface_kind() == SurfaceKind::MetabossHarness
    {
        return M6LegacyFixtureFencePosture::SyntheticEndToEndBlocked;
    }

    M6LegacyFixtureFencePosture::SyntheticEndToEndBlocked
}
