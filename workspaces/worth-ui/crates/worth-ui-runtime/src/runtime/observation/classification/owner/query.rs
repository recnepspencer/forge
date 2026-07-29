use crate::fact_contract::{UiProducedFact, UiQueryChangedFact};

pub(in crate::runtime::observation::classification) fn classify(
    observation: worth_ui_query_binding::WorthUiValidatedCollectionChangeObservation,
) -> UiProducedFact {
    UiProducedFact::Query(UiQueryChangedFact::from_owner_consequence(
        observation.into_consequence(),
    ))
}
