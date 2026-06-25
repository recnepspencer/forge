use super::*;

#[test]
fn px_lengths_canonicalize_equivalent_decimal_forms() {
    assert_eq!(
        WorthUiLengthValue::from_px("12px").unwrap(),
        WorthUiLengthValue::from_px("12.0px").unwrap()
    );
    assert_eq!(
        WorthUiLengthValue::from_px("12.34px").unwrap().points(),
        12.34
    );
}

#[test]
fn padding_shorthand_canonicalizes_to_explicit_edges() {
    assert_eq!(
        WorthUiPaddingValue::from_shorthand_px("4px 8px").unwrap(),
        WorthUiPaddingValue::from_shorthand_px("4px 8px 4px 8px").unwrap()
    );
}

#[test]
fn invalid_units_and_negative_lengths_are_rejected() {
    assert!(WorthUiLengthValue::from_px("12").is_err());
    assert!(WorthUiLengthValue::from_px("-1px").is_err());
    assert!(WorthUiPaddingValue::from_shorthand_px("4em").is_err());
}
