use crate::fact_contract::UiProducedFact;

pub(in crate::runtime::observation::classification) fn classify(
    observation: crate::mounting::UiIntentPostureObservation,
) -> UiProducedFact {
    let (graph_node, target, reference, posture) = observation.into_parts();
    UiProducedFact::IntentPosture(crate::fact_contract::UiIntentPostureChangedFact::new(
        graph_node, target, reference, posture,
    ))
}
