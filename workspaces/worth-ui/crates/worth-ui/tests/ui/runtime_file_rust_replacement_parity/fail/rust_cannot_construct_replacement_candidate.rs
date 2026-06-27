use worth_ui::facade::{WorthUiCandidateAuthoringLane, WorthUiReplacementCandidate};

fn main() {
    let _candidate = WorthUiReplacementCandidate {
        bundle: uninitialized_field(),
        cause: uninitialized_field(),
        provenance_handle: uninitialized_field(),
        authoring_lane: WorthUiCandidateAuthoringLane::RustAuthored,
    };
}

fn uninitialized_field<T>() -> T {
    unimplemented!()
}
