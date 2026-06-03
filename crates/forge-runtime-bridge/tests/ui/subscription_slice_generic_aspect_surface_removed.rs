use forge_runtime_bridge::facade::{
    NormalizedSubscriptionSliceIntent, SubscriptionSliceKind, TruthDeltaSurfaceKind,
};

fn main() {
    let aspect_key = forge_foundational::facade::AspectKey::new("profile").unwrap();
    let _intent = NormalizedSubscriptionSliceIntent::try_new_aspect_surface(
        "entity-1",
        aspect_key,
        TruthDeltaSurfaceKind::EntityRegion,
        SubscriptionSliceKind::SignalRegion,
    );
}
