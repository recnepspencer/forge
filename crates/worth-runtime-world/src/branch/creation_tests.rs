use super::{
    ProductBranchComponentPosture, ProductBranchComponentPostures, ProductBranchCreationIntent,
    ProductBranchNameDenial,
};

#[test]
fn component_postures_expose_explicit_reuse_and_owner_effect_meaning() {
    assert!(ProductBranchComponentPosture::ReuseExact.is_reuse_exact());
    assert!(!ProductBranchComponentPosture::ReuseExact.requires_owner_effect());
    assert!(ProductBranchComponentPosture::ForkExact.requires_owner_effect());
    assert!(ProductBranchComponentPosture::ForkAndAdvance.requires_owner_effect());

    let reuse = ProductBranchComponentPostures::new(
        ProductBranchComponentPosture::ReuseExact,
        ProductBranchComponentPosture::ReuseExact,
    );
    assert!(reuse.is_exact_reuse());
    assert!(!reuse.requires_owner_effect());

    let mixed = ProductBranchComponentPostures::new(
        ProductBranchComponentPosture::ForkExact,
        ProductBranchComponentPosture::ReuseExact,
    );
    assert!(!mixed.is_exact_reuse());
    assert!(mixed.requires_owner_effect());
}

#[test]
fn branch_creation_name_is_validated_before_identity_issuance() {
    assert_eq!(
        ProductBranchCreationIntent::named("   ").expect_err("blank names are not meaning"),
        ProductBranchNameDenial::Empty
    );
    assert!(ProductBranchCreationIntent::named("child").is_ok());
    assert!(ProductBranchCreationIntent::named("x".repeat(257)).is_err());
}
