use worth_primitives::{truth_digest_parts, TruthDigestScope};

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarBooleanSharedEndpointEvent {
    shared_endpoint_event_identity: String,
    source_endpoint_identities: Vec<String>,
    endpoint_projection_fact_digests: Vec<String>,
    carrier_identities: Vec<String>,
}

impl PlanarBooleanSharedEndpointEvent {
    pub(crate) fn new(
        source_endpoint_identities: Vec<String>,
        endpoint_projection_fact_digests: Vec<String>,
        carrier_identities: Vec<String>,
    ) -> Self {
        let source_endpoint_identities = canonical_values(source_endpoint_identities);
        let endpoint_projection_fact_digests = canonical_values(endpoint_projection_fact_digests);
        let carrier_identities = canonical_values(carrier_identities);
        let shared_endpoint_event_identity = identity(
            &source_endpoint_identities,
            &endpoint_projection_fact_digests,
            &carrier_identities,
        );
        Self {
            shared_endpoint_event_identity,
            source_endpoint_identities,
            endpoint_projection_fact_digests,
            carrier_identities,
        }
    }

    pub(crate) fn merge_with(&mut self, other: &Self) {
        append_canonical(
            &mut self.source_endpoint_identities,
            &other.source_endpoint_identities,
        );
        append_canonical(
            &mut self.endpoint_projection_fact_digests,
            &other.endpoint_projection_fact_digests,
        );
        append_canonical(&mut self.carrier_identities, &other.carrier_identities);
        self.shared_endpoint_event_identity = identity(
            &self.source_endpoint_identities,
            &self.endpoint_projection_fact_digests,
            &self.carrier_identities,
        );
    }

    pub fn shared_endpoint_event_identity(&self) -> &str {
        &self.shared_endpoint_event_identity
    }

    pub fn source_endpoint_identities(&self) -> &[String] {
        &self.source_endpoint_identities
    }

    pub fn endpoint_projection_fact_digests(&self) -> &[String] {
        &self.endpoint_projection_fact_digests
    }

    pub fn carrier_identities(&self) -> &[String] {
        &self.carrier_identities
    }
}

fn identity(
    source_endpoint_identities: &[String],
    endpoint_projection_fact_digests: &[String],
    carrier_identities: &[String],
) -> String {
    let mut parts = vec!["planar-boolean-shared-endpoint-event".to_string()];
    parts.extend(
        source_endpoint_identities
            .iter()
            .map(|identity| format!("source-endpoint:{identity}")),
    );
    parts.extend(
        endpoint_projection_fact_digests
            .iter()
            .map(|digest| format!("endpoint-projection:{digest}")),
    );
    parts.extend(
        carrier_identities
            .iter()
            .map(|identity| format!("carrier:{identity}")),
    );
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, &parts)
}

fn append_canonical(target: &mut Vec<String>, source: &[String]) {
    target.extend(source.iter().cloned());
    *target = canonical_values(std::mem::take(target));
}

fn canonical_values(mut values: Vec<String>) -> Vec<String> {
    values.sort();
    values.dedup();
    values
}
