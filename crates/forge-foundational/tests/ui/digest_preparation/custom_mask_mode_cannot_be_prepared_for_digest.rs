struct CustomMaskMode;

fn main() {
    let mask = forge_foundational::AspectMask::<CustomMaskMode>::whole_aspect();

    forge_foundational::prepare_aspect_mask_for_digest(
        forge_foundational::AspectKey::new("counter").unwrap(),
        mask,
    );
}
