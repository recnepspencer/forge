use worth_query::facade::runtime::{WorthQueryAuthorityLane, WorthQueryEffectHandle, WorthQueryUnrefinedLiveShape};
use std::marker::PhantomData;

fn main() {
    let _worthd: WorthQueryEffectHandle<WorthQueryUnrefinedLiveShape> = WorthQueryEffectHandle {
        name: "ui.Worthd".to_string(),
        authority_lane: WorthQueryAuthorityLane::EffectDeliveryState,
        marker: PhantomData,
    };
}
