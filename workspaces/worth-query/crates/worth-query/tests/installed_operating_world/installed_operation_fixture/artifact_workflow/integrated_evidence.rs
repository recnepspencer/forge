use worth_foundational::facade::FoundationalPerformanceCounterName;
use worth_query::facade::domain;

use super::native_observation::ArtifactNativeSuccess;

pub(super) fn integrated_evidence(
    native: &ArtifactNativeSuccess,
) -> domain::WorthQueryDomainEvidenceMaterial {
    let counters = native.evidence().counters();
    domain::WorthQueryDomainEvidenceMaterial::new()
        .counter(counter_observation(
            "artifact-bytes",
            counters.source_bytes as u64,
        ))
        .counter(counter_observation(
            "artifact-elements",
            counters.values_exposed as u64,
        ))
        .counter(counter_observation(
            "artifact-work",
            counters
                .provider_contacts
                .saturating_add(counters.values_exposed) as u64,
        ))
}

fn counter_observation(
    name: &str,
    observed: u64,
) -> domain::WorthQueryStructuralCounterObservation {
    domain::WorthQueryStructuralCounterObservation::new(
        FoundationalPerformanceCounterName::new(name).unwrap(),
        0,
        observed,
    )
}
