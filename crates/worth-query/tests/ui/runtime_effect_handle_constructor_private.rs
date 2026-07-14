use worth_query::facade::runtime::{WorthQueryAuthorityLane, WorthQueryEffectHandle, WorthQueryNativeRow};
use std::marker::PhantomData;

fn main() {
    let _worthd: WorthQueryEffectHandle<WorthQueryNativeRow> = WorthQueryEffectHandle {
        name: "ui.Worthd".to_string(),
        authority_lane: WorthQueryAuthorityLane::EffectDeliveryState,
        marker: PhantomData,
    };
}
