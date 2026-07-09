use crate::diagnostics::data::RelationalDiagnosticValue;
use crate::durability::data::{DurableCheckpointId, DurableSegmentId};
use crate::replay::data::ReplayVerificationLayer;
use crate::schema::data::ContractId;

pub(super) fn verification_layer_value(
    layer: ReplayVerificationLayer,
) -> RelationalDiagnosticValue {
    RelationalDiagnosticValue::string(format!("{layer:?}"))
}

pub(super) fn checkpoint_id_array(
    checkpoint_ids: &[DurableCheckpointId],
) -> RelationalDiagnosticValue {
    RelationalDiagnosticValue::array(
        checkpoint_ids
            .iter()
            .copied()
            .map(RelationalDiagnosticValue::DurableCheckpointId),
    )
}

pub(super) fn segment_id_array(segment_ids: &[DurableSegmentId]) -> RelationalDiagnosticValue {
    RelationalDiagnosticValue::array(
        segment_ids
            .iter()
            .copied()
            .map(RelationalDiagnosticValue::DurableSegmentId),
    )
}

pub(super) fn contract_id_array(contract_ids: &[ContractId]) -> RelationalDiagnosticValue {
    RelationalDiagnosticValue::array(
        contract_ids
            .iter()
            .cloned()
            .map(RelationalDiagnosticValue::ContractId),
    )
}
