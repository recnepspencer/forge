use serde_json::{Map, Value};

pub(super) type HarnessSummaryValue = Value;

pub(super) fn harness_summary_object(
    fields: impl IntoIterator<Item = (&'static str, HarnessSummaryValue)>,
) -> HarnessSummaryValue {
    Value::Object(Map::from_iter(
        fields
            .into_iter()
            .map(|(field, value)| (field.to_string(), value)),
    ))
}

pub(super) fn harness_summary_array(
    values: impl IntoIterator<Item = HarnessSummaryValue>,
) -> HarnessSummaryValue {
    Value::Array(values.into_iter().collect())
}

pub(super) fn harness_summary_projected_value(value: Value) -> HarnessSummaryValue {
    value
}

pub(super) fn harness_summary_string(value: impl Into<String>) -> HarnessSummaryValue {
    Value::String(value.into())
}

pub(super) fn harness_summary_u64(value: u64) -> HarnessSummaryValue {
    Value::from(value)
}

pub(super) fn harness_summary_usize(value: usize) -> HarnessSummaryValue {
    Value::from(value as u64)
}

pub(super) fn harness_summary_bool(value: bool) -> HarnessSummaryValue {
    Value::Bool(value)
}

pub(super) fn optional_harness_summary_u64(value: Option<u64>) -> HarnessSummaryValue {
    value.map(Value::from).unwrap_or(Value::Null)
}

pub(super) fn optional_harness_summary_usize(value: Option<usize>) -> HarnessSummaryValue {
    value
        .map(|count| Value::from(count as u64))
        .unwrap_or(Value::Null)
}

pub(super) fn optional_harness_summary_string(value: Option<String>) -> HarnessSummaryValue {
    value.map(Value::String).unwrap_or(Value::Null)
}
