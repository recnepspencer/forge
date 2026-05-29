use forge_foundational::facade::{
    AspectFieldLocator, AspectKey, AspectLocator, AspectValueLocator, CanonicalFieldPath, FieldKey,
    LocatorAuthority,
};

use super::{decode_aspect_value_locator, encode_aspect_value_locator};

#[test]
fn aspect_value_locator_codec_roundtrips_whole_aspect_and_struct_field() {
    for locator in [
        AspectValueLocator::whole_aspect(AspectLocator::new(
            LocatorAuthority::Authoritative,
            AspectKey::new("deploy.replicas").expect("valid aspect key"),
        )),
        AspectValueLocator::struct_field(AspectFieldLocator::new(
            LocatorAuthority::ReceiptBearing,
            AspectKey::new("deploy.config").expect("valid aspect key"),
            CanonicalFieldPath::new(vec![
                FieldKey::new("routing").expect("valid field key"),
                FieldKey::new("weight").expect("valid field key"),
            ])
            .expect("valid field path"),
        )),
    ] {
        let encoded = encode_aspect_value_locator(&locator);
        let decoded = decode_aspect_value_locator(&encoded).expect("decode locator bytes");

        assert_eq!(decoded, locator);
    }
}

#[test]
fn aspect_value_locator_codec_rejects_malformed_bytes() {
    let malformed_locator = vec![99, 1];

    let error = decode_aspect_value_locator(&malformed_locator)
        .expect_err("malformed locator bytes should fail");

    assert!(
        error
            .to_string()
            .contains("unknown aspect value locator tag")
            || error.to_string().contains("input ended early"),
        "unexpected locator codec error: {error}"
    );
}
