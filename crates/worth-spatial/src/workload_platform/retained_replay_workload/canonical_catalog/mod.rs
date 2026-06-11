use super::{
    CapturedRetainedWorkload, RetainedWorkload, UnsupportedReplayReasonCode,
    UnsupportedReplayWorkload,
};
use planar_bundle::{
    canonical_planar_bundle_parts, projection_consumed_canonical_planar_facts,
    retained_canonical_planar_facts,
};

mod planar_bundle;
mod planar_receipts;
mod query_handles;

pub(super) const TOPOLOGY: &str = "topology:canonical-retained-cancellation";
pub(super) const MOVEMENT: &str = "movement:canonical-retained-cancellation";
pub(super) const NEIGHBORHOOD: &str = "neighborhood:canonical-retained-cancellation";

pub fn canonical_retained_cancellation_chain_capture(
    world: &'static str,
) -> Result<CapturedRetainedWorkload, UnsupportedReplayWorkload> {
    let bundle = canonical_planar_bundle_parts(world)?;
    let retained = retained_canonical_planar_facts(world, &bundle)?;
    let projection_consumed =
        projection_consumed_canonical_planar_facts(world, &bundle, &retained)?;

    RetainedWorkload::from_retained_planar_facts(retained)
        .declared(format!(
            "capture canonical retained cancellation chain artifacts for {world}"
        ))
        .with_projection_consumed_facts(projection_consumed)
        .capture()
}

pub(crate) fn canonical_retained_replay_error(
    human_reason: impl Into<String>,
) -> UnsupportedReplayWorkload {
    UnsupportedReplayWorkload::new(
        UnsupportedReplayReasonCode::MissingRetainedArtifacts,
        human_reason,
    )
}
