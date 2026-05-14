fn main() {
    let mask = forge_foundational::AspectMask::<forge_foundational::MutationMask>::new([
        forge_foundational::CanonicalFieldPath::single(
            forge_foundational::FieldKey::new("count").unwrap(),
        ),
    ]);

    forge_foundational::aspect_mask_digest_preparation_basis(&mask);
}
