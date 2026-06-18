#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanEdgeSplitCloseoutDecisionRow {
    decision_identity: String,
    phase_name: String,
    decision_kind_name: String,
    source_edge_identity: String,
    carrier_identity: String,
    affected_artifact_identity: String,
    event_identities: Vec<String>,
    event_group_identities: Vec<String>,
    upstream_receipt_identity: String,
}

impl PlanarBooleanEdgeSplitCloseoutDecisionRow {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        decision_identity: String,
        phase_name: String,
        decision_kind_name: String,
        source_edge_identity: String,
        carrier_identity: String,
        affected_artifact_identity: String,
        event_identities: Vec<String>,
        event_group_identities: Vec<String>,
        upstream_receipt_identity: String,
    ) -> Self {
        Self {
            decision_identity,
            phase_name,
            decision_kind_name,
            source_edge_identity,
            carrier_identity,
            affected_artifact_identity,
            event_identities,
            event_group_identities,
            upstream_receipt_identity,
        }
    }

    pub fn decision_identity(&self) -> &str {
        &self.decision_identity
    }
    pub fn phase_name(&self) -> &str {
        &self.phase_name
    }
    pub fn decision_kind_name(&self) -> &str {
        &self.decision_kind_name
    }
    pub fn source_edge_identity(&self) -> &str {
        &self.source_edge_identity
    }
    pub fn carrier_identity(&self) -> &str {
        &self.carrier_identity
    }
    pub fn affected_artifact_identity(&self) -> &str {
        &self.affected_artifact_identity
    }
    pub fn event_identities(&self) -> &[String] {
        &self.event_identities
    }
    pub fn event_group_identities(&self) -> &[String] {
        &self.event_group_identities
    }
    pub fn upstream_receipt_identity(&self) -> &str {
        &self.upstream_receipt_identity
    }
}

pub(crate) fn closeout_decision_localization_rows(
    rows: &[super::super::PlanarBooleanSplitDecisionRow],
) -> Vec<PlanarBooleanEdgeSplitCloseoutDecisionRow> {
    rows.iter()
        .map(|row| {
            PlanarBooleanEdgeSplitCloseoutDecisionRow::new(
                row.decision_identity().to_string(),
                format!("{:?}", row.phase()),
                format!("{:?}", row.kind()),
                row.source_edge_identity().to_string(),
                row.carrier_identity().to_string(),
                row.affected_artifact_identity().to_string(),
                row.event_identities().to_vec(),
                row.event_group_identities().to_vec(),
                row.upstream_receipt_identity().to_string(),
            )
        })
        .collect()
}
