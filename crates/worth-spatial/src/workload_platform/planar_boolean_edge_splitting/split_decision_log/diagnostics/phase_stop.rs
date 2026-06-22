use super::affected_artifact::PlanarBooleanSplitAffectedArtifact;
use super::decision_reason::PlanarBooleanSplitDecisionReason;
use super::identity::decision_identity;
use super::kind::PlanarBooleanSplitDecisionKind;
use super::phase::PlanarBooleanSplitDecisionPhase;
use super::row::PlanarBooleanSplitDecisionRow;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanEdgeSplitPhaseStop {
    stop_identity: String,
    phase: PlanarBooleanSplitDecisionPhase,
    source_edge_identity: String,
    carrier_identity: String,
    evidence_identity: String,
    event_identities: Vec<String>,
    event_group_identities: Vec<String>,
    denial_kind: String,
    human_reason: String,
}

impl PlanarBooleanEdgeSplitPhaseStop {
    #[cfg(test)]
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn typed_denial(
        phase: PlanarBooleanSplitDecisionPhase,
        source_edge_identity: impl Into<String>,
        carrier_identity: impl Into<String>,
        evidence_identity: impl Into<String>,
        event_identities: Vec<String>,
        event_group_identities: Vec<String>,
        denial_kind: impl Into<String>,
        human_reason: impl Into<String>,
    ) -> Self {
        let source_edge_identity = source_edge_identity.into();
        let carrier_identity = carrier_identity.into();
        let evidence_identity = evidence_identity.into();
        let denial_kind = denial_kind.into();
        let stop_identity = format!(
            "edge-split-phase-stop:{}:{}:{}:{}",
            phase.as_str(),
            source_edge_identity,
            carrier_identity,
            evidence_identity
        );
        Self {
            stop_identity,
            phase,
            source_edge_identity,
            carrier_identity,
            evidence_identity,
            event_identities,
            event_group_identities,
            denial_kind,
            human_reason: human_reason.into(),
        }
    }

    pub(crate) fn to_decision_row(&self) -> PlanarBooleanSplitDecisionRow {
        let row_identity = decision_identity(
            PlanarBooleanSplitDecisionPhase::PhaseStop,
            PlanarBooleanSplitDecisionKind::SplitPhaseDenied,
            PlanarBooleanSplitAffectedArtifact::PhaseStop,
            &self.stop_identity,
            &self.evidence_identity,
        );
        PlanarBooleanSplitDecisionRow::new(
            row_identity,
            self.phase,
            PlanarBooleanSplitDecisionKind::SplitPhaseDenied,
            PlanarBooleanSplitAffectedArtifact::PhaseStop,
            self.stop_identity.clone(),
            self.source_edge_identity.clone(),
            self.carrier_identity.clone(),
            self.event_identities.clone(),
            self.event_group_identities.clone(),
            vec![self.evidence_identity.clone()],
            self.evidence_identity.clone(),
            PlanarBooleanSplitDecisionReason::SplitPhaseDenied(self.denial_kind.clone()),
            Some(self.denial_kind.clone()),
        )
    }

    pub fn stop_identity(&self) -> &str {
        &self.stop_identity
    }
    pub fn phase(&self) -> PlanarBooleanSplitDecisionPhase {
        self.phase
    }
    pub fn evidence_identity(&self) -> &str {
        &self.evidence_identity
    }
    pub fn denial_kind(&self) -> &str {
        &self.denial_kind
    }
    pub fn human_reason(&self) -> &str {
        &self.human_reason
    }
}
