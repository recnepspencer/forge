pub(super) fn string_aspect_value(
    value: impl Into<String>,
) -> crate::runtime::WorthQueryAuthoredAspectValue {
    crate::runtime::WorthQueryAuthoredAspectValue::string(value)
}
