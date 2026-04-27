use forge_query::facade::{
    ForgeQueryAuthorityLane, ForgeQueryRuntimeStateKind, ForgeQueryRuntimeStateSnapshot,
};

fn main() {
    let _forged = ForgeQueryRuntimeStateSnapshot {
        kind: ForgeQueryRuntimeStateKind::Ready,
        basis_digest: String::new(),
        result_shape_digest: String::new(),
        authority_lane: ForgeQueryAuthorityLane::AuthoritativeTruth,
        explanation: String::new(),
        state_digest: String::new(),
    };
}
