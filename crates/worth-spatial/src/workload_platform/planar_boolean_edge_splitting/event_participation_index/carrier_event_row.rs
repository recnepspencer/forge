use super::identity::participation_row_identity;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanSplitEventParticipationRow {
    participation_row_identity: String,
    carrier_identity: String,
    source_edge_identity: String,
    start_source_endpoint_identity: String,
    start_projected_endpoint_fact_identity: String,
    end_source_endpoint_identity: String,
    end_projected_endpoint_fact_identity: String,
    point_event_identities: Vec<String>,
    interval_event_identities: Vec<String>,
    event_group_identities: Vec<String>,
}

impl PlanarBooleanSplitEventParticipationRow {
    pub(crate) fn new(
        event_ledger_identity: &str,
        carrier_identity: impl Into<String>,
        source_edge_identity: impl Into<String>,
        start_source_endpoint_identity: impl Into<String>,
        start_projected_endpoint_fact_identity: impl Into<String>,
        end_source_endpoint_identity: impl Into<String>,
        end_projected_endpoint_fact_identity: impl Into<String>,
        point_event_identities: Vec<String>,
        interval_event_identities: Vec<String>,
        event_group_identities: Vec<String>,
    ) -> Self {
        let carrier_identity = carrier_identity.into();
        let source_edge_identity = source_edge_identity.into();
        let start_source_endpoint_identity = start_source_endpoint_identity.into();
        let start_projected_endpoint_fact_identity = start_projected_endpoint_fact_identity.into();
        let end_source_endpoint_identity = end_source_endpoint_identity.into();
        let end_projected_endpoint_fact_identity = end_projected_endpoint_fact_identity.into();
        let point_event_identities = canonical_values(point_event_identities);
        let interval_event_identities = canonical_values(interval_event_identities);
        let event_group_identities = canonical_values(event_group_identities);
        let participation_row_identity = participation_row_identity(
            event_ledger_identity,
            &carrier_identity,
            &source_edge_identity,
            &start_source_endpoint_identity,
            &start_projected_endpoint_fact_identity,
            &end_source_endpoint_identity,
            &end_projected_endpoint_fact_identity,
            &point_event_identities,
            &interval_event_identities,
            &event_group_identities,
        );
        Self {
            participation_row_identity,
            carrier_identity,
            source_edge_identity,
            start_source_endpoint_identity,
            start_projected_endpoint_fact_identity,
            end_source_endpoint_identity,
            end_projected_endpoint_fact_identity,
            point_event_identities,
            interval_event_identities,
            event_group_identities,
        }
    }

    pub fn participation_row_identity(&self) -> &str {
        &self.participation_row_identity
    }

    pub fn carrier_identity(&self) -> &str {
        &self.carrier_identity
    }

    pub fn source_edge_identity(&self) -> &str {
        &self.source_edge_identity
    }

    pub fn start_source_endpoint_identity(&self) -> &str {
        &self.start_source_endpoint_identity
    }

    pub fn start_projected_endpoint_fact_identity(&self) -> &str {
        &self.start_projected_endpoint_fact_identity
    }

    pub fn end_source_endpoint_identity(&self) -> &str {
        &self.end_source_endpoint_identity
    }

    pub fn end_projected_endpoint_fact_identity(&self) -> &str {
        &self.end_projected_endpoint_fact_identity
    }

    pub fn point_event_identities(&self) -> &[String] {
        &self.point_event_identities
    }

    pub fn interval_event_identities(&self) -> &[String] {
        &self.interval_event_identities
    }

    pub fn event_group_identities(&self) -> &[String] {
        &self.event_group_identities
    }
}

fn canonical_values(mut values: Vec<String>) -> Vec<String> {
    values.sort();
    values.dedup();
    values
}
