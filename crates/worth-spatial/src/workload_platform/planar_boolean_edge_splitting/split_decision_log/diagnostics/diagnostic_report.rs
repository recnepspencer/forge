use super::identity::diagnostic_report_identity;
use super::localization::PlanarBooleanSplitFailureLocalization;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanStructuredEdgeSplitFailureReport {
    report_identity: String,
    localization_identity: String,
    decision_identity: String,
    phase_name: String,
    decision_kind_name: String,
    source_edge_identity: String,
    carrier_identity: String,
    affected_artifact_identity: String,
    event_identities: Vec<String>,
    event_group_identities: Vec<String>,
    machine_reason: String,
}

impl PlanarBooleanStructuredEdgeSplitFailureReport {
    pub(crate) fn from_localization(localization: &PlanarBooleanSplitFailureLocalization) -> Self {
        let report_identity = diagnostic_report_identity(
            localization.localization_identity(),
            localization.decision_identity(),
        );
        let machine_reason = localization
            .policy_or_denial_kind()
            .unwrap_or(localization.kind().as_str())
            .to_string();
        Self {
            report_identity,
            localization_identity: localization.localization_identity().to_string(),
            decision_identity: localization.decision_identity().to_string(),
            phase_name: localization.phase().as_str().to_string(),
            decision_kind_name: localization.kind().as_str().to_string(),
            source_edge_identity: localization.source_edge_identity().to_string(),
            carrier_identity: localization.carrier_identity().to_string(),
            affected_artifact_identity: localization.affected_artifact_identity().to_string(),
            event_identities: localization.event_identities().to_vec(),
            event_group_identities: localization.event_group_identities().to_vec(),
            machine_reason,
        }
    }
    pub fn report_identity(&self) -> &str {
        &self.report_identity
    }
    pub fn localization_identity(&self) -> &str {
        &self.localization_identity
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
    pub fn machine_reason(&self) -> &str {
        &self.machine_reason
    }
}
