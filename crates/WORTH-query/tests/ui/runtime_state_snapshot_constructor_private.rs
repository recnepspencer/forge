use worth_query::facade::{
    WorthQueryAuthorityLane, WorthQueryRuntimeStateKind, WorthQueryRuntimeStateSnapshot,
};

fn main() {
    let _worthd = WorthQueryRuntimeStateSnapshot {
        kind: WorthQueryRuntimeStateKind::Ready,
        basis_digest: String::new(),
        result_shape_digest: String::new(),
        authority_lane: WorthQueryAuthorityLane::AuthoritativeTruth,
        explanation: String::new(),
        state_digest: String::new(),
    };
}
