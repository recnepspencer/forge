use worth_query::facade::consumer_kit::{
    WorthQueryPinnedSupportStatus, WorthQueryPinnedTeachingPosture,
    WorthQueryRuntimeFacadeFamily, WorthQuerySupportPinRequirement,
};

fn main() {
    let _ = WorthQuerySupportPinRequirement {
        family: WorthQueryRuntimeFacadeFamily::Write,
        surface: String::new(),
        required_status: WorthQueryPinnedSupportStatus::Supported,
        required_teaching_posture: WorthQueryPinnedTeachingPosture::OrdinaryRuntimeDx,
        pinned_live_row_digest: String::new(),
        pinned_snapshot_row_digest: String::new(),
    };
}
