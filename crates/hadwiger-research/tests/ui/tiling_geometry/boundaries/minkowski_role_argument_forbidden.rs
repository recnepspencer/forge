use hadwiger_research::facade::{
    evaluate_tiling_minkowski_contact_checked, HadwigerResearchHandle, TilingCell,
    TilingContactRole,
};

fn main() {
    fn attempt(handle: &HadwigerResearchHandle, cell: &TilingCell) {
        let _ = evaluate_tiling_minkowski_contact_checked(
            handle,
            cell,
            "tile-a",
            "tile-b",
            TilingContactRole::SameColorConflictCandidate,
        );
    }
}
