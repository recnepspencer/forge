use worth_query::facade::runtime::{WorthQueryAuthorityLane, WorthQueryDerivedViewHandle, WorthQueryNativeRow};

fn main() {
    let _worthd = WorthQueryDerivedViewHandle::<WorthQueryNativeRow> {
        name: "Worthd.computed".to_string(),
        authority_lane: WorthQueryAuthorityLane::DerivedRuntimeState,
        marker: std::marker::PhantomData,
    };
}
