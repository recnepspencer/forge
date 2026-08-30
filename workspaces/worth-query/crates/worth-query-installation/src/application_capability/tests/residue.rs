#[test]
fn warm_capability_lookup_contains_no_canonical_or_digest_work() {
    let warm_lookup = include_str!("../installed_contract.rs");
    let registry = include_str!("../registry.rs");
    let warm_identity_path = [warm_lookup, registry].join("\n");
    for forbidden in [
        "prepare_capability_basis",
        "prepare_canonical_basis_sequence",
        "canonicalization()",
        "encode_digest",
        "Sha256",
        "sha2::",
    ] {
        assert!(
            !warm_lookup.contains(forbidden),
            "warm capability lookup contains forbidden work: {forbidden}"
        );
    }
    assert!(warm_lookup.contains(".capability_registry"));
    assert!(warm_lookup.contains(".get(&key)"));
    assert!(warm_lookup.contains(".cloned()"));
    assert!(warm_lookup.contains("WorthQueryCanonicalWorkEvidence::zero()"));
    for forbidden in [
        "std::any::type_name",
        "core::any::type_name",
        "type_name::<",
    ] {
        assert!(
            !warm_identity_path.contains(forbidden),
            "warm capability identity contains Rust type-path residue: {forbidden}"
        );
    }
}

#[test]
fn capability_identity_uses_foundational_digest_and_central_authority_transcript() {
    let canonical = include_str!("../canonical_basis.rs");
    let authority = include_str!("../authority_seal.rs");
    let registry = include_str!("../registry.rs");
    let combined = [canonical, authority, registry].join("\n");

    for forbidden in [
        "use sha2",
        "Sha256::",
        "DefaultHasher",
        "format!(\"{:?}\"",
        ".hash(",
    ] {
        assert!(
            !combined.contains(forbidden),
            "capability identity contains a private identity grammar: {forbidden}"
        );
    }
    assert!(canonical.contains("prepare_canonical_basis_sequence"));
    assert!(canonical.contains("CanonicalDigestAlgorithmId::sha256()"));
    assert!(authority.contains("AuthorityTranscript::new"));
    assert!(authority.contains("InstalledApplicationCapability"));
    assert!(registry.contains("prepare_capability_basis"));
}
