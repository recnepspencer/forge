use crate::capability::CapabilitySnapshotDigest;
use crate::runtime::WorthUiExecutionPlanDigest;
use crate::source::WorthUiArtifactDigest;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct WorthUiActiveExecutionPlan {
    digest: WorthUiActiveExecutionPlanDigest,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct WorthUiActiveExecutionPlanDigest {
    value: u64,
}

impl WorthUiActiveExecutionPlan {
    pub(crate) fn from_launch_authority(
        artifact_digest: WorthUiArtifactDigest,
        snapshot_digest: CapabilitySnapshotDigest,
    ) -> Self {
        Self {
            digest: WorthUiActiveExecutionPlanDigest::from_launch_authority(
                artifact_digest,
                snapshot_digest,
            ),
        }
    }

    pub(crate) fn digest(self) -> WorthUiActiveExecutionPlanDigest {
        self.digest
    }

    pub(crate) fn from_swap_authority(candidate_digest: WorthUiExecutionPlanDigest) -> Self {
        Self {
            digest: WorthUiActiveExecutionPlanDigest {
                value: candidate_digest.raw(),
            },
        }
    }
}

impl WorthUiActiveExecutionPlanDigest {
    fn from_launch_authority(
        artifact_digest: WorthUiArtifactDigest,
        snapshot_digest: CapabilitySnapshotDigest,
    ) -> Self {
        Self {
            value: 0xa11c_e119_5eed_0001
                ^ artifact_digest.raw().rotate_left(11)
                ^ snapshot_digest.as_u64().rotate_left(37),
        }
    }

    pub(crate) fn as_u64(self) -> u64 {
        self.value
    }
}
