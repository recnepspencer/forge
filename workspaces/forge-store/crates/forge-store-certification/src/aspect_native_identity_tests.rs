use forge_foundational::{
    aspects, AspectFieldLocator, AspectLocator, AspectValueLocator, CanonicalFieldPath, FieldKey,
    LocatorAuthority,
};
use forge_store_aspect_native::{
    StoreAspectBoundaryLocator, StoreAspectFieldBoundaryLocator, StoreAspectIdentity,
    StoreAspectNativeDenial, StoreAspectValueBoundaryLocator,
};

#[test]
fn store_aspect_keys_are_admitted_not_parsed_from_paths() {
    let aspect_key = aspects()
        .vocabulary()
        .key("store.physical.segment.identity")
        .unwrap();
    let identity = StoreAspectIdentity::from_aspect_key(aspect_key.clone());
    let aspect_locator = AspectLocator::new(LocatorAuthority::Authoritative, aspect_key.clone());
    let store_locator =
        StoreAspectBoundaryLocator::new(identity.clone(), aspect_locator.clone()).unwrap();
    let field_key = FieldKey::new("segment").unwrap();
    let field_locator = AspectFieldLocator::from_aspect(
        aspect_locator.clone(),
        CanonicalFieldPath::single(field_key),
    );
    let store_field_locator =
        StoreAspectFieldBoundaryLocator::new(identity.clone(), field_locator.clone()).unwrap();
    let store_value_locator = StoreAspectValueBoundaryLocator::new(
        identity.clone(),
        AspectValueLocator::struct_field(field_locator),
    )
    .unwrap();

    assert_eq!(store_locator.identity(), &identity);
    assert_eq!(store_locator.locator().aspect_key(), identity.aspect_key());
    assert_eq!(store_field_locator.identity(), &identity);
    assert_eq!(store_value_locator.identity(), &identity);
}

#[test]
fn raw_unadmitted_strings_are_rejected_before_aspect_key_admission() {
    let rejected = aspects().vocabulary().key("store physical segment");

    assert!(rejected.is_err());
}

#[test]
fn store_locators_reject_mismatched_aspect_identity() {
    let identity_key = aspects()
        .vocabulary()
        .key("store.physical.segment.identity")
        .unwrap();
    let other_key = aspects()
        .vocabulary()
        .key("store.physical.segment.header")
        .unwrap();
    let identity = StoreAspectIdentity::from_aspect_key(identity_key);
    let other_locator = AspectLocator::new(LocatorAuthority::Authoritative, other_key.clone());

    assert_eq!(
        StoreAspectBoundaryLocator::new(identity.clone(), other_locator.clone()),
        Err(StoreAspectNativeDenial::LocatorIdentityMismatch)
    );

    let field_locator = AspectFieldLocator::from_aspect(
        other_locator.clone(),
        CanonicalFieldPath::single(FieldKey::new("segment").unwrap()),
    );
    assert_eq!(
        StoreAspectFieldBoundaryLocator::new(identity.clone(), field_locator.clone()),
        Err(StoreAspectNativeDenial::LocatorIdentityMismatch)
    );
    assert_eq!(
        StoreAspectValueBoundaryLocator::new(
            identity,
            AspectValueLocator::struct_field(field_locator),
        ),
        Err(StoreAspectNativeDenial::LocatorIdentityMismatch)
    );
}
