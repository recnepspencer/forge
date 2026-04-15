use forge_relational::facade::runtime::RelationalRuntime;
use worth_schema::facade::VerifiedTopologyCommit;

use crate::certification::{
    WorthMilestoneOneCertificationError, WorthMilestoneOneCertificationHarness,
    WorthMilestoneOneCertificationReport,
};

pub(crate) fn certified_verified_commit(
    runtime: &mut RelationalRuntime,
    verified: &VerifiedTopologyCommit,
) -> Result<WorthMilestoneOneCertificationReport, WorthMilestoneOneCertificationError> {
    WorthMilestoneOneCertificationHarness::certify_verified_commit(runtime, verified)
}
