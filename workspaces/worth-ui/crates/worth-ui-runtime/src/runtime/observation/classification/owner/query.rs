use crate::fact_contract::{UiProducedFact, UiQueryChangedFact};
use crate::runtime::observation::turn::UiAdmittedQueryObservation;

pub(in crate::runtime::observation::classification) fn classify(
    observation: UiAdmittedQueryObservation,
) -> UiProducedFact {
    let fact = match observation {
        UiAdmittedQueryObservation::OperationLive(observation) => {
            UiQueryChangedFact::from_owner_consequence(observation.into_consequence())
        }
        UiAdmittedQueryObservation::Projection(observation) => {
            UiQueryChangedFact::from_projection_observation(observation)
        }
    };
    UiProducedFact::Query(fact)
}
