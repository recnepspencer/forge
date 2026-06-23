use worth_spatial::facade::planar_boolean_edge_splitting::{
    PlanarBooleanSplitAffectedArtifact, PlanarBooleanSplitDecisionKind,
    PlanarBooleanSplitDecisionPhase, PlanarBooleanSplitFailureLocalization,
};

fn main() {
    let _localization = PlanarBooleanSplitFailureLocalization {
        localization_identity: "localization".to_string(),
        decision_identity: "decision".to_string(),
        phase: PlanarBooleanSplitDecisionPhase::PhaseStop,
        kind: PlanarBooleanSplitDecisionKind::SplitPhaseDenied,
        affected_artifact: PlanarBooleanSplitAffectedArtifact::PhaseStop,
        affected_artifact_identity: "artifact".to_string(),
        source_edge_identity: "source edge".to_string(),
        carrier_identity: "carrier".to_string(),
        event_identities: Vec::new(),
        event_group_identities: Vec::new(),
        policy_or_denial_kind: Some("denial".to_string()),
    };
}
