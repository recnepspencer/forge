use forge_query::facade::{ForgeQueryAuthorityLane, ForgeQueryEffectHandle};
use serde_json::Value;
use std::marker::PhantomData;

fn main() {
    let _forged: ForgeQueryEffectHandle<Value> = ForgeQueryEffectHandle {
        name: "ui.forged".to_string(),
        authority_lane: ForgeQueryAuthorityLane::EffectDeliveryState,
        marker: PhantomData,
    };
}
