use worth_foundational::facade::{AspectKey, FieldKey};

fn aspect_key(value: &str) -> AspectKey {
    AspectKey::new(value).expect("test aspect key should be valid")
}

fn field_key(value: &str) -> FieldKey {
    FieldKey::new(value).expect("test field key should be valid")
}

mod artifact;
mod collection_patch;
mod delivery_policy;
mod detail_patch;
mod locality;
mod materialization_patch;
mod progress;
mod promotion;
mod replay;
mod stream_delivery;
