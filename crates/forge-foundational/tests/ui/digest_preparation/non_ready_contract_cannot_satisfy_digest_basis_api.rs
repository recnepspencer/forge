fn main() {
    let contract = forge_foundational::AspectContract::scalar(
        forge_foundational::AspectKey::new("counter").unwrap(),
        forge_foundational::AspectIdentity(1),
        forge_foundational::AspectContractRevision(1),
        forge_foundational::ScalarAspectType::Int64,
    );

    forge_foundational::aspect_contract_digest_preparation_basis(&contract);
}
