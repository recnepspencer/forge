use forge_query::facade::consumer_kit::{
    ForgeQueryPinnedSupportStatus, ForgeQueryPinnedTeachingPosture,
    ForgeQueryRuntimeFacadeFamily, ForgeQuerySupportPinRequirement,
};

fn main() {
    let _ = ForgeQuerySupportPinRequirement {
        family: ForgeQueryRuntimeFacadeFamily::Write,
        surface: String::new(),
        required_status: ForgeQueryPinnedSupportStatus::Supported,
        required_teaching_posture: ForgeQueryPinnedTeachingPosture::OrdinaryRuntimeDx,
        pinned_live_row_digest: String::new(),
        pinned_snapshot_row_digest: String::new(),
    };
}
