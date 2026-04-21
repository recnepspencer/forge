use super::admission::{
    CompatibilityAdmissionCounters, CompatibilityEdgeRegistry, CompatibilityRejection,
    CompatibilityRejectionKind, CompatibilityRelation, ReaderCapabilitySet,
    UpgradeAdmissionWitness, WriterCapabilitySet,
};
use super::manifests::{ArtifactCompatibilityWindow, ArtifactFamilyId, ArtifactSemanticVersion};
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RollingUpgradeWindow {
    family_id: ArtifactFamilyId,
    window: ArtifactCompatibilityWindow,
}

impl RollingUpgradeWindow {
    pub fn new(family_id: ArtifactFamilyId, window: ArtifactCompatibilityWindow) -> Self {
        Self { family_id, window }
    }

    pub fn family_id(&self) -> &ArtifactFamilyId {
        &self.family_id
    }

    pub fn window(&self) -> &ArtifactCompatibilityWindow {
        &self.window
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct MixedVersionStorePosture {
    family_id: ArtifactFamilyId,
    posture: MixedVersionPostureKind,
}

impl MixedVersionStorePosture {
    pub fn new(family_id: ArtifactFamilyId, posture: impl Into<String>) -> Self {
        Self {
            family_id,
            posture: MixedVersionPostureKind::LegacyLabel(posture.into()),
        }
    }

    pub(crate) fn admitted(family_id: ArtifactFamilyId) -> Self {
        Self {
            family_id,
            posture: MixedVersionPostureKind::AdmittedTwoCapabilityWindow,
        }
    }

    pub(crate) fn rejected(family_id: ArtifactFamilyId) -> Self {
        Self {
            family_id,
            posture: MixedVersionPostureKind::RejectedUnsupportedSkew,
        }
    }

    pub fn posture(&self) -> &MixedVersionPostureKind {
        &self.posture
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ReplicaCompatibilityPosture {
    family_id: ArtifactFamilyId,
    posture: MixedVersionPostureKind,
}

impl ReplicaCompatibilityPosture {
    pub fn new(family_id: ArtifactFamilyId, posture: impl Into<String>) -> Self {
        Self {
            family_id,
            posture: MixedVersionPostureKind::LegacyLabel(posture.into()),
        }
    }

    pub(crate) fn admitted(family_id: ArtifactFamilyId) -> Self {
        Self {
            family_id,
            posture: MixedVersionPostureKind::AdmittedTwoCapabilityWindow,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct MaintenanceCompatibilityPosture {
    family_id: ArtifactFamilyId,
    posture: String,
}

impl MaintenanceCompatibilityPosture {
    pub fn new(family_id: ArtifactFamilyId, posture: impl Into<String>) -> Self {
        Self {
            family_id,
            posture: posture.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum MixedVersionPostureKind {
    AdmittedTwoCapabilityWindow,
    RejectedUnsupportedSkew,
    LegacyLabel(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum RollingUpgradePolicy {
    FirstShipTwoCapability,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RollingCapabilityWindow {
    family_id: ArtifactFamilyId,
    reader_versions: Vec<ArtifactSemanticVersion>,
    writer_versions: Vec<ArtifactSemanticVersion>,
}

impl RollingCapabilityWindow {
    fn new(reader: &ReaderCapabilitySet, writer: &WriterCapabilitySet) -> Self {
        Self {
            family_id: reader.family_id().clone(),
            reader_versions: reader.semantic_versions().to_vec(),
            writer_versions: writer.semantic_versions().to_vec(),
        }
    }

    pub fn family_id(&self) -> &ArtifactFamilyId {
        &self.family_id
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RollingUpgradeAdmissionPlan {
    policy: RollingUpgradePolicy,
    window: RollingUpgradeWindow,
    capability_window: RollingCapabilityWindow,
    relation: CompatibilityRelation,
    store_posture: MixedVersionStorePosture,
    replica_posture: ReplicaCompatibilityPosture,
    witness: UpgradeAdmissionWitness,
}

impl RollingUpgradeAdmissionPlan {
    pub(crate) fn new(
        policy: RollingUpgradePolicy,
        window: RollingUpgradeWindow,
        capability_window: RollingCapabilityWindow,
        relation: CompatibilityRelation,
        witness: UpgradeAdmissionWitness,
    ) -> Self {
        let family_id = window.family_id().clone();
        Self {
            policy,
            window,
            capability_window,
            relation,
            store_posture: MixedVersionStorePosture::admitted(family_id.clone()),
            replica_posture: ReplicaCompatibilityPosture::admitted(family_id),
            witness,
        }
    }

    pub fn policy(&self) -> RollingUpgradePolicy {
        self.policy
    }

    pub fn store_posture(&self) -> &MixedVersionStorePosture {
        &self.store_posture
    }

    pub fn relation(&self) -> CompatibilityRelation {
        self.relation
    }

    pub(crate) fn witness(&self) -> &UpgradeAdmissionWitness {
        &self.witness
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RollingUpgradeRejection {
    family_id: ArtifactFamilyId,
    reason: String,
}

impl RollingUpgradeRejection {
    pub(crate) fn new(family_id: ArtifactFamilyId, reason: impl Into<String>) -> Self {
        Self {
            family_id,
            reason: reason.into(),
        }
    }
}

pub(crate) fn plan_first_ship_rolling_upgrade(
    counters: &mut CompatibilityAdmissionCounters,
    edge_registry: &CompatibilityEdgeRegistry,
    window: &RollingUpgradeWindow,
    readers: &[ReaderCapabilitySet],
    writers: &[WriterCapabilitySet],
) -> Result<RollingUpgradeAdmissionPlan, CompatibilityRejection> {
    if readers.len() != 1 || writers.len() != 1 {
        if writers.len() > 1 {
            counters.record_rolling_multi_writer_rejection();
            return Err(CompatibilityRejection::new(
                CompatibilityRejectionKind::RollingMultiWriterRejected,
                window.family_id().clone(),
                "first-ship rolling policy admits exactly one writer capability set",
            ));
        }
        counters.record_rolling_window_rejection();
        return Err(CompatibilityRejection::new(
            CompatibilityRejectionKind::RollingWindowRejected,
            window.family_id().clone(),
            "first-ship rolling policy admits exactly one reader and one writer capability set",
        ));
    }

    let reader = &readers[0];
    let writer = &writers[0];
    if reader.family_id() != window.family_id() || writer.family_id() != window.family_id() {
        counters.record_rolling_window_rejection();
        return Err(CompatibilityRejection::new(
            CompatibilityRejectionKind::RollingWindowRejected,
            window.family_id().clone(),
            "rolling capability family must match the declared rolling window",
        ));
    }

    let Some(reader_version) = single_capability_version(reader.semantic_versions()) else {
        counters.record_rolling_window_rejection();
        return Err(CompatibilityRejection::new(
            CompatibilityRejectionKind::RollingWindowRejected,
            window.family_id().clone(),
            "first-ship rolling policy admits exactly one reader semantic version",
        ));
    };
    let Some(writer_version) = single_capability_version(writer.semantic_versions()) else {
        counters.record_rolling_window_rejection();
        return Err(CompatibilityRejection::new(
            CompatibilityRejectionKind::RollingWindowRejected,
            window.family_id().clone(),
            "first-ship rolling policy admits exactly one writer semantic version",
        ));
    };

    if !capability_versions_inside_window(reader.semantic_versions(), window.window())
        || !capability_versions_inside_window(writer.semantic_versions(), window.window())
    {
        counters.record_mixed_version_skew();
        counters.record_rolling_window_rejection();
        let _posture = MixedVersionStorePosture::rejected(window.family_id().clone());
        let _rejection = RollingUpgradeRejection::new(
            window.family_id().clone(),
            "reader/writer capability versions are outside the rolling window",
        );
        return Err(CompatibilityRejection::new(
            CompatibilityRejectionKind::MixedVersionSkewRejected,
            window.family_id().clone(),
            "reader/writer capability versions are outside the rolling window",
        ));
    }

    counters.record_relation_recheck();
    let Some(edge) = edge_registry.get(window.family_id(), reader_version, writer_version) else {
        counters.record_edge_missing_rejection();
        counters.record_rolling_window_rejection();
        return Err(CompatibilityRejection::new(
            CompatibilityRejectionKind::MissingCompatibilityEdge,
            window.family_id().clone(),
            "declared rolling compatibility edge is missing",
        ));
    };
    let relation = edge.relation();
    match relation {
        CompatibilityRelation::Native
        | CompatibilityRelation::ForwardRead
        | CompatibilityRelation::BackwardRead => {}
        CompatibilityRelation::AdapterRequired
        | CompatibilityRelation::DerivedRebuildRequired
        | CompatibilityRelation::Incompatible => {
            counters.record_rolling_window_rejection();
            return Err(CompatibilityRejection::new(
                CompatibilityRejectionKind::RollingWindowRejected,
                window.family_id().clone(),
                "first-ship rolling policy rejects adapter, rebuild, and incompatible edges",
            ));
        }
    }

    counters.record_rolling_window_admission();
    Ok(RollingUpgradeAdmissionPlan::new(
        RollingUpgradePolicy::FirstShipTwoCapability,
        window.clone(),
        RollingCapabilityWindow::new(reader, writer),
        relation,
        UpgradeAdmissionWitness::new(window.family_id().clone()),
    ))
}

fn single_capability_version(
    versions: &[ArtifactSemanticVersion],
) -> Option<ArtifactSemanticVersion> {
    match versions {
        [version] => Some(*version),
        _ => None,
    }
}

fn capability_versions_inside_window(
    versions: &[ArtifactSemanticVersion],
    window: &ArtifactCompatibilityWindow,
) -> bool {
    !versions.is_empty()
        && versions
            .iter()
            .all(|version| window.contains_semantic(*version))
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct UpgradeSkewRejection {
    family_id: ArtifactFamilyId,
    reason: String,
}

impl UpgradeSkewRejection {
    pub fn new(family_id: ArtifactFamilyId, reason: impl Into<String>) -> Self {
        Self {
            family_id,
            reason: reason.into(),
        }
    }
}
