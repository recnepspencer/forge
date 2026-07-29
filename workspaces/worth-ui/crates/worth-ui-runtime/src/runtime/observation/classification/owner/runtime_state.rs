use crate::fact_contract::{
    UiCommittedPortalAnchorChangedFact, UiCommittedScrollExtentChangedFact, UiProducedFact,
};

pub(in crate::runtime::observation::classification) fn classify_scroll(
    observation: super::super::super::admission::UiCommittedScrollExtentObservation,
) -> UiProducedFact {
    let (revision, sources) = observation.into_parts();
    UiProducedFact::CommittedScrollExtent(UiCommittedScrollExtentChangedFact::new(
        revision, sources,
    ))
}

pub(in crate::runtime::observation::classification) fn classify_portal(
    observation: super::super::super::admission::UiCommittedPortalAnchorObservation,
) -> UiProducedFact {
    let (revision, sources) = observation.into_parts();
    UiProducedFact::CommittedPortalAnchor(UiCommittedPortalAnchorChangedFact::new(
        revision, sources,
    ))
}
