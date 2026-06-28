use crate::{PartialPublicationEvidence, PartialPublicationObservationSet};

use super::PartialPublicationObservedSource;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PartialPublicationObservationAdmission {
    admitted_source: PartialPublicationObservedSource,
}

impl PartialPublicationObservationAdmission {
    pub fn admit_observations(observations: PartialPublicationObservationSet) -> Self {
        let sources = observations.into_sources();
        let admitted_source = select_recovery_authoritative_observation(sources);
        Self { admitted_source }
    }

    pub(crate) fn into_evidence(self) -> PartialPublicationEvidence {
        self.admitted_source.into_evidence()
    }
}

fn select_recovery_authoritative_observation(
    sources: super::observation_set::PartialPublicationObservationSources,
) -> PartialPublicationObservedSource {
    sources
        .torn_publication
        .or(sources.durable_page_mutation)
        .or(sources.persisted_crash_edge)
        .or(sources.backend_residue)
        .or(sources.live_ack_memory)
        .or(sources.log_only)
        .or(sources.insufficient_persisted_evidence)
        .unwrap_or_else(|| {
            PartialPublicationObservedSource::insufficient_persisted_evidence(
                "partial-publication:no-observations",
            )
        })
}
