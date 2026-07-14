use std::collections::BTreeMap;

use serde::{de::DeserializeOwned, Serialize};
use serde_json::{Map as ExternalRecordObject, Value as ExternalRecordValue};

pub type HarnessRecordSummaryValue = ExternalRecordValue;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HarnessSummaryProjection {
    Null,
    Bool(bool),
    Unsigned(u64),
    Signed(i64),
    String(String),
    Array(Vec<HarnessSummaryProjection>),
    Object(BTreeMap<String, HarnessSummaryProjection>),
}

impl HarnessSummaryProjection {
    pub fn object(
        fields: impl IntoIterator<Item = (impl Into<String>, HarnessSummaryProjection)>,
    ) -> Self {
        Self::Object(
            fields
                .into_iter()
                .map(|(field, value)| (field.into(), value))
                .collect(),
        )
    }

    pub fn array(values: impl IntoIterator<Item = HarnessSummaryProjection>) -> Self {
        Self::Array(values.into_iter().collect())
    }

    pub fn into_record_summary_value(self) -> ExternalRecordValue {
        match self {
            Self::Null => ExternalRecordValue::Null,
            Self::Bool(value) => ExternalRecordValue::Bool(value),
            Self::Unsigned(value) => ExternalRecordValue::from(value),
            Self::Signed(value) => ExternalRecordValue::from(value),
            Self::String(value) => ExternalRecordValue::String(value),
            Self::Array(values) => ExternalRecordValue::Array(
                values
                    .into_iter()
                    .map(HarnessSummaryProjection::into_record_summary_value)
                    .collect(),
            ),
            Self::Object(fields) => ExternalRecordValue::Object(ExternalRecordObject::from_iter(
                fields
                    .into_iter()
                    .map(|(field, value)| (field, value.into_record_summary_value())),
            )),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct HarnessRecordSummaryView<'summary> {
    root: &'summary HarnessRecordSummaryValue,
}

impl<'summary> HarnessRecordSummaryView<'summary> {
    pub fn new(root: &'summary HarnessRecordSummaryValue) -> Self {
        Self { root }
    }

    pub fn string_field(&self, field: &str) -> Option<&'summary str> {
        self.root.get(field)?.as_str()
    }

    pub fn string_field_at(&self, path: &[&str]) -> Option<&'summary str> {
        self.value_at(path)?.as_str()
    }

    pub fn u64_field_at(&self, path: &[&str]) -> Option<u64> {
        self.value_at(path)?.as_u64()
    }

    pub fn is_array_at(&self, path: &[&str]) -> bool {
        self.value_at(path)
            .is_some_and(HarnessRecordSummaryValue::is_array)
    }

    pub fn object_array_at(&self, path: &[&str]) -> Vec<Self> {
        self.value_at(path)
            .and_then(HarnessRecordSummaryValue::as_array)
            .into_iter()
            .flatten()
            .map(Self::new)
            .collect()
    }

    fn value_at(&self, path: &[&str]) -> Option<&'summary HarnessRecordSummaryValue> {
        path.iter()
            .try_fold(self.root, |value, segment| value.get(*segment))
    }
}

pub fn record_summary_from_serializable<T>(
    value: &T,
) -> Result<HarnessRecordSummaryValue, serde_json::Error>
where
    T: Serialize,
{
    serde_json::to_value(value)
}

pub fn record_summary_into_deserializable<T>(
    value: HarnessRecordSummaryValue,
) -> Result<T, serde_json::Error>
where
    T: DeserializeOwned,
{
    serde_json::from_value(value)
}

impl From<bool> for HarnessSummaryProjection {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<u64> for HarnessSummaryProjection {
    fn from(value: u64) -> Self {
        Self::Unsigned(value)
    }
}

impl From<i64> for HarnessSummaryProjection {
    fn from(value: i64) -> Self {
        Self::Signed(value)
    }
}

impl From<String> for HarnessSummaryProjection {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for HarnessSummaryProjection {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    use super::{
        record_summary_from_serializable, record_summary_into_deserializable,
        HarnessSummaryProjection,
    };

    #[test]
    fn record_summary_value_materializes_nested_projection_at_harness_boundary() {
        let projection = HarnessSummaryProjection::object([
            ("status", HarnessSummaryProjection::from("validated")),
            (
                "counts",
                HarnessSummaryProjection::array([
                    HarnessSummaryProjection::Unsigned(2),
                    HarnessSummaryProjection::Unsigned(3),
                ]),
            ),
        ]);

        let record_summary = projection.into_record_summary_value();

        assert_eq!(record_summary["status"], "validated");
        assert_eq!(record_summary["counts"][0], 2);
        assert_eq!(record_summary["counts"][1], 3);
    }

    #[test]
    fn record_summary_view_reads_nested_external_record_values_without_exposing_json() {
        let projection = HarnessSummaryProjection::object([
            ("status", HarnessSummaryProjection::from("validated")),
            (
                "counters",
                HarnessSummaryProjection::object([("hits", HarnessSummaryProjection::Unsigned(9))]),
            ),
            (
                "entries",
                HarnessSummaryProjection::array([HarnessSummaryProjection::object([(
                    "code",
                    HarnessSummaryProjection::from("Ready"),
                )])]),
            ),
        ]);
        let record_summary = projection.into_record_summary_value();
        let view = super::HarnessRecordSummaryView::new(&record_summary);

        assert_eq!(view.string_field("status"), Some("validated"));
        assert_eq!(view.u64_field_at(&["counters", "hits"]), Some(9));
        assert_eq!(
            view.object_array_at(&["entries"])[0].string_field("code"),
            Some("Ready")
        );
        assert_eq!(view.object_array_at(&["entries"]).len(), 1);
    }

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    struct SerializableSummaryRecord {
        status: String,
        counts: Vec<u64>,
    }

    #[test]
    fn serializable_record_summary_materialization_stays_at_harness_boundary() {
        let summary_record = SerializableSummaryRecord {
            status: "validated".to_string(),
            counts: vec![2, 3],
        };

        let record_summary =
            record_summary_from_serializable(&summary_record).expect("record summary value");
        let recovered: SerializableSummaryRecord =
            record_summary_into_deserializable(record_summary.clone())
                .expect("recovered summary record");
        let view = super::HarnessRecordSummaryView::new(&record_summary);

        assert_eq!(recovered, summary_record);
        assert_eq!(view.string_field("status"), Some("validated"));
        assert!(view.is_array_at(&["counts"]));
    }
}
