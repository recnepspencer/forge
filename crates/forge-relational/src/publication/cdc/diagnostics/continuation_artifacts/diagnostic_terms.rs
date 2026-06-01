use crate::diagnostics::data::RelationalDiagnosticValue;
use crate::publication::cdc::data::{
    SubscriberContinuationClassSet, SubscriberStrataSet, SubscriberStreamFailureClass,
};
use crate::schema::data::{SchemaContinuationClassification, SchemaStratum};

pub(super) fn continuation_class_set_value(
    classes: &SubscriberContinuationClassSet,
) -> RelationalDiagnosticValue {
    RelationalDiagnosticValue::array(
        classes
            .iter()
            .copied()
            .map(continuation_classification_value),
    )
}

pub(super) fn strata_set_value(strata: &SubscriberStrataSet) -> RelationalDiagnosticValue {
    RelationalDiagnosticValue::array(strata.iter().copied().map(stratum_value))
}

pub(super) fn strata_slice_value(strata: &[SchemaStratum]) -> RelationalDiagnosticValue {
    RelationalDiagnosticValue::array(strata.iter().copied().map(stratum_value))
}

pub(super) fn continuation_classification_value(
    classification: SchemaContinuationClassification,
) -> RelationalDiagnosticValue {
    RelationalDiagnosticValue::string(format!("{classification:?}"))
}

fn stratum_value(stratum: SchemaStratum) -> RelationalDiagnosticValue {
    RelationalDiagnosticValue::string(format!("{stratum:?}"))
}

pub(super) fn failure_class_value(
    class: SubscriberStreamFailureClass,
) -> RelationalDiagnosticValue {
    RelationalDiagnosticValue::string(format!("{class:?}"))
}
