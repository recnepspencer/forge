use crate::merge::logic::policy::contexts::ValueLookupFailure;
use forge_foundational::facade::{
    AspectKey, AspectValue, AuthoritativeRecordAspectState, ContractValidatedAspectValueView,
};

pub(super) fn scalar_from_authoritative_state(
    authoritative_state: Option<&AuthoritativeRecordAspectState>,
    aspect_key: &AspectKey,
) -> Result<AspectValue, ValueLookupFailure> {
    let Some(entry) = authoritative_state.and_then(|state| state.get(aspect_key)) else {
        return Err(ValueLookupFailure::MissingField);
    };
    match entry.view() {
        ContractValidatedAspectValueView::Scalar(value) => Ok(value.clone()),
        ContractValidatedAspectValueView::Struct(_) => Err(ValueLookupFailure::InvalidValueShape),
    }
}

pub(super) fn aspect_value_i64(value: AspectValue) -> Result<i64, ValueLookupFailure> {
    match value {
        AspectValue::Int8(value) => Ok(i64::from(value)),
        AspectValue::Int16(value) => Ok(i64::from(value)),
        AspectValue::Int32(value) => Ok(i64::from(value)),
        AspectValue::Int64(value) => Ok(value),
        AspectValue::UInt8(value) => Ok(i64::from(value)),
        AspectValue::UInt16(value) => Ok(i64::from(value)),
        AspectValue::UInt32(value) => Ok(i64::from(value)),
        AspectValue::UInt64(value) => {
            i64::try_from(value).map_err(|_| ValueLookupFailure::InvalidValueShape)
        }
        _ => Err(ValueLookupFailure::InvalidValueShape),
    }
}
