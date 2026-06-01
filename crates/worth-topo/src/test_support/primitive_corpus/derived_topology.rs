use forge_relational::facade::runtime::RelationalRuntime;

use crate::certification::{
    MilestoneOneCertificationError, MilestoneOneCertificationHarness,
    MilestoneOneCertificationReport,
};
use crate::committed_artifact::TopologyCommittedArtifact;

pub(crate) fn certified_verified_commit(
    runtime: &mut RelationalRuntime,
    verified: &TopologyCommittedArtifact,
) -> Result<MilestoneOneCertificationReport, MilestoneOneCertificationError> {
    MilestoneOneCertificationHarness::certify_verified_commit(runtime, verified)
}
