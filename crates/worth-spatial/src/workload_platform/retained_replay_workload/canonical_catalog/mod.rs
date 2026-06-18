use super::{
    CapturedRetainedWorkload, RetainedWorkload, UnsupportedReplayReasonCode,
    UnsupportedReplayWorkload,
};
use planar_bundle::{
    canonical_planar_bundle_parts, projection_consumed_canonical_planar_facts,
    retained_canonical_planar_facts,
};
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex, OnceLock};

mod planar_bundle;
mod planar_receipts;
mod query_handles;

pub(super) const TOPOLOGY: &str = "topology:canonical-retained-cancellation";
pub(super) const MOVEMENT: &str = "movement:canonical-retained-cancellation";
pub(super) const NEIGHBORHOOD: &str = "neighborhood:canonical-retained-cancellation";

type CaptureResult = Result<CapturedRetainedWorkload, UnsupportedReplayWorkload>;

fn retained_capture_cache() -> &'static Mutex<BTreeMap<&'static str, Arc<OnceLock<CaptureResult>>>>
{
    static CACHE: OnceLock<Mutex<BTreeMap<&'static str, Arc<OnceLock<CaptureResult>>>>> =
        OnceLock::new();
    CACHE.get_or_init(|| Mutex::new(BTreeMap::new()))
}

pub fn canonical_retained_cancellation_chain_capture(
    world: &'static str,
) -> Result<CapturedRetainedWorkload, UnsupportedReplayWorkload> {
    let entry = {
        let mut cache = retained_capture_cache()
            .lock()
            .expect("canonical retained capture cache lock should not be poisoned");
        cache
            .entry(world)
            .or_insert_with(|| Arc::new(OnceLock::new()))
            .clone()
    };
    entry
        .get_or_init(|| build_canonical_retained_cancellation_chain_capture(world))
        .clone()
}

fn build_canonical_retained_cancellation_chain_capture(
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
