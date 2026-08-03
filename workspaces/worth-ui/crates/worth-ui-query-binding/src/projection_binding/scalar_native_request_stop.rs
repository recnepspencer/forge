use super::UiProjectionBindingStopKind;

pub(super) fn scalar_native_request_stop(
    denial: crate::application_binding::WorthUiScalarTextNativeRequestDenial,
) -> (UiProjectionBindingStopKind, String) {
    use crate::application_binding::WorthUiScalarTextNativeRequestDenial as Denial;
    use worth_query::facade::installed::operation::WorthQueryNativeProjectionRequestDenialKind;
    match denial {
        Denial::ProjectionRequest(denial) => {
            use WorthQueryNativeProjectionRequestDenialKind as Kind;
            let kind = match denial.kind() {
                Kind::WholeAspectNotProjected
                | Kind::UnknownField
                | Kind::FieldNotProjected
                | Kind::ConflictingDeclaration => UiProjectionBindingStopKind::SchemaMismatch,
                Kind::FieldRequiresStruct | Kind::UnsupportedAspectShape | Kind::NoNativeFacts => {
                    UiProjectionBindingStopKind::NativeFamilyMismatch
                }
            };
            (
                kind,
                format!(
                    "Query rejected the scalar native request: {:?}",
                    denial.kind()
                ),
            )
        }
        Denial::SelectionMismatch(denial) => (
            UiProjectionBindingStopKind::SchemaMismatch,
            format!(
                "Query rejected the scalar selected-field key: {:?}",
                denial.kind()
            ),
        ),
    }
}
