use crate::application_binding::WorthUiSettledScalarTextProjection;

#[derive(Debug, Eq, PartialEq)]
pub struct WorthUiScalarNativeKeyReport {
    contract_key: String,
    contract_identity: u64,
    contract_revision: u64,
    field_path: String,
    expected_shape: String,
    absence_posture: String,
    lane: String,
    contract_digest: String,
}

pub(super) fn project_scalar_native_key(
    settled: &WorthUiSettledScalarTextProjection,
) -> WorthUiScalarNativeKeyReport {
    let key = settled.certification_native_key();
    WorthUiScalarNativeKeyReport {
        contract_key: key.contract_key().as_str().to_owned(),
        contract_identity: key.contract_identity().0,
        contract_revision: key.contract_revision().0,
        field_path: format!("{:?}", key.field_path()),
        expected_shape: format!("{:?}", key.expected_shape()),
        absence_posture: format!("{:?}", key.absence_posture()),
        lane: format!("{:?}", key.lane()),
        contract_digest: settled.certification_contract_digest(),
    }
}
