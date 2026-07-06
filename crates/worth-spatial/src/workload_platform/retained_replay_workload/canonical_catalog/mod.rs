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
use std::time::Instant;

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
    trace_scope("canonical_retained_capture_build", || {
        let bundle = trace_scope("canonical_planar_bundle_parts", || {
            canonical_planar_bundle_parts(world)
        })?;
        let retained = trace_scope("retained_canonical_planar_facts", || {
            retained_canonical_planar_facts(world, &bundle)
        })?;
        let projection_consumed =
            trace_scope("projection_consumed_canonical_planar_facts", || {
                projection_consumed_canonical_planar_facts(world, &bundle, &retained)
            })?;
        trace_scope("retained_workload_capture", || {
            RetainedWorkload::from_retained_planar_facts(retained)
                .declared(format!(
                    "capture canonical retained cancellation chain artifacts for {world}"
                ))
                .with_projection_consumed_facts(projection_consumed)
                .capture()
        })
    })
}

pub(crate) fn canonical_retained_replay_error(
    human_reason: impl Into<String>,
) -> UnsupportedReplayWorkload {
    UnsupportedReplayWorkload::new(
        UnsupportedReplayReasonCode::MissingRetainedArtifacts,
        human_reason,
    )
}

pub(super) fn trace_scope<T>(label: &str, action: impl FnOnce() -> T) -> T {
    if !trace_enabled() {
        return action();
    }
    eprintln!("[worth-perf]       start {label}");
    let start = Instant::now();
    let result = action();
    eprintln!(
        "[worth-perf]       finish {label} ({:.3}s)",
        start.elapsed().as_secs_f64()
    );
    result
}

fn trace_enabled() -> bool {
    static ENABLED: OnceLock<bool> = OnceLock::new();
    *ENABLED.get_or_init(|| std::env::var_os("WORTH_TRACE_PERFORMANCE").is_some())
}
