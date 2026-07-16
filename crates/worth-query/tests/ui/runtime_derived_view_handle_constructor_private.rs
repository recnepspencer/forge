use worth_query::facade::runtime::{WorthQueryAuthorityLane, WorthQueryDerivedViewHandle, WorthQueryUnrefinedLiveShape};

fn main() {
    let _worthd = WorthQueryDerivedViewHandle::<WorthQueryUnrefinedLiveShape> {
        name: "Worthd.computed".to_string(),
        authority_lane: WorthQueryAuthorityLane::DerivedRuntimeState,
        marker: std::marker::PhantomData,
    };
}
