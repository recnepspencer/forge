use crate::authoring::AspectFieldKey;

pub(crate) fn test_declaration_aspect_key(value: &str) -> AspectFieldKey {
    let (aspect, field) = value.split_once('.').expect("test aspect key shape");
    AspectFieldKey::new(aspect, field).expect("valid test aspect key")
}

pub(crate) fn test_declaration_aspect_keys(values: &[&str]) -> Vec<AspectFieldKey> {
    values
        .iter()
        .map(|value| test_declaration_aspect_key(value))
        .collect()
}

pub(crate) fn test_declaration_aspect_projections(fields: &[AspectFieldKey]) -> Vec<String> {
    fields
        .iter()
        .map(super::declaration_aspect::terminal_declaration_aspect_projection)
        .collect()
}

pub(crate) fn assert_declaration_aspect_projections(fields: &[AspectFieldKey], expected: &[&str]) {
    assert_eq!(
        test_declaration_aspect_projections(fields),
        expected
            .iter()
            .map(|value| value.to_string())
            .collect::<Vec<_>>()
    );
}
