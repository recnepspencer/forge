use serde::{Deserialize, Serialize};
use smallvec::SmallVec;

use crate::data::output::CanonicalChangedRegions;
use crate::data::reuse::{ReuseBoundaryContext, ReuseCertificationRecord};

use super::records::RetainedDiagnosticArtifact;
use super::tiers::RuntimeArtifactState;

pub(crate) const COLD_ARTIFACT_INTENT_LABEL_LIMIT: usize = 4;

/// Explicit write packet for artifact hot/cold lanes.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct ArtifactWriteDelta {
    #[serde(default)]
    pub runtime: Option<RuntimeArtifactState>,
    #[serde(default)]
    pub retained: Option<RetainedDiagnosticArtifact>,
}

/// Explicit hot-lane write packet for runtime artifact state updates.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct HotArtifactWrite {
    #[serde(default)]
    pub runtime: Option<RuntimeArtifactState>,
    #[serde(default)]
    pub cold_intent: Option<ColdArtifactIntent>,
}

/// Bounded cold-path seed emitted by the hot execution lane.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct ColdArtifactIntent {
    #[serde(default)]
    pub changed_regions: CanonicalChangedRegions,
    #[serde(default)]
    pub labels: SmallVec<[String; COLD_ARTIFACT_INTENT_LABEL_LIMIT]>,
    #[serde(default)]
    pub keyed_family: Option<String>,
    #[serde(default)]
    pub keyed_key: Option<String>,
    #[serde(default)]
    pub reuse_certification: Option<ReuseCertificationRecord>,
    #[serde(default)]
    pub reuse_boundary_context: Option<ReuseBoundaryContext>,
}

/// Cold retained record kept off the operational hot path.
pub type ColdArtifactRecord = RetainedDiagnosticArtifact;

impl ColdArtifactIntent {
    pub fn is_empty(&self) -> bool {
        self.changed_regions.is_empty()
            && self.labels.is_empty()
            && self.keyed_family.is_none()
            && self.keyed_key.is_none()
            && self.reuse_certification.is_none()
            && self.reuse_boundary_context.is_none()
    }

    pub fn materialize_record(self) -> Option<ColdArtifactRecord> {
        if self.is_empty() {
            return None;
        }
        Some(ColdArtifactRecord {
            changed_regions: self.changed_regions,
            labels: self.labels.into_vec(),
            keyed_family: self.keyed_family,
            keyed_key: self.keyed_key,
            reuse_certification: self.reuse_certification,
            reuse_boundary_context: self.reuse_boundary_context,
        })
    }
}
