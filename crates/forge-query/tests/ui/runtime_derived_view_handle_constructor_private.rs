use forge_query::facade::{ForgeQueryAuthorityLane, ForgeQueryDerivedViewHandle};

fn main() {
    let _forged = ForgeQueryDerivedViewHandle::<serde_json::Value> {
        name: "forged.computed".to_string(),
        authority_lane: ForgeQueryAuthorityLane::DerivedRuntimeState,
        marker: std::marker::PhantomData,
    };
}
