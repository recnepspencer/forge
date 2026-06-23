use forge_query::facade::{ForgeQueryAuthorityLane, ForgeQueryEffectHandle, ForgeQueryNativeRow};
use std::marker::PhantomData;

fn main() {
    let _forged: ForgeQueryEffectHandle<ForgeQueryNativeRow> = ForgeQueryEffectHandle {
        name: "ui.forged".to_string(),
        authority_lane: ForgeQueryAuthorityLane::EffectDeliveryState,
        marker: PhantomData,
    };
}
