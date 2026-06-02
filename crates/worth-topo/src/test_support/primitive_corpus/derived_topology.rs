use crate::certification::support::commit_certification_input::TopologyCommitCertificationInput;
use forge_relational::facade::runtime::RelationalRuntime;

use crate::certification::{
    MilestoneOneCertificationError, MilestoneOneCertificationHarness,
    MilestoneOneCertificationReport,
};

pub(crate) fn certified_topology_commit_input(
    runtime: &mut RelationalRuntime,
    commit_input: &TopologyCommitCertificationInput,
) -> Result<MilestoneOneCertificationReport, MilestoneOneCertificationError> {
    MilestoneOneCertificationHarness::certify_commit_input(runtime, commit_input)
}
