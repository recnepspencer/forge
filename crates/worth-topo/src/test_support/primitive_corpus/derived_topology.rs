use forge_relational::facade::runtime::RelationalRuntime;
use schema::facade::VerifiedTopologyCommit;

use crate::certification::{
    MilestoneOneCertificationError, MilestoneOneCertificationHarness,
    MilestoneOneCertificationReport,
};

pub(crate) fn certified_verified_commit(
    runtime: &mut RelationalRuntime,
    verified: &VerifiedTopologyCommit,
) -> Result<MilestoneOneCertificationReport, MilestoneOneCertificationError> {
    MilestoneOneCertificationHarness::certify_verified_commit(runtime, verified)
}
