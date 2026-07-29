use crate::fact_contract::{UiMeasurementChangedFact, UiProducedFact};

pub(in crate::runtime::observation::classification) fn classify(
    result: crate::host_exchange::measurement_admission::UiSolicitedHostMeasurementResult,
) -> UiProducedFact {
    UiProducedFact::Measurement(UiMeasurementChangedFact::new(result))
}
