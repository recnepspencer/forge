use worth_spatial::facade::planar_boolean_edge_splitting::{
    PlanarBooleanSplitAffectedArtifact, PlanarBooleanSplitDecisionKind,
    PlanarBooleanSplitDecisionPhase, PlanarBooleanSplitDecisionReason, PlanarBooleanSplitDecisionRow,
};

fn main() {
    let _row = PlanarBooleanSplitDecisionRow::new(
        "decision".to_string(),
        PlanarBooleanSplitDecisionPhase::PhaseStop,
        PlanarBooleanSplitDecisionKind::SplitPhaseDenied,
        PlanarBooleanSplitAffectedArtifact::PhaseStop,
        "artifact".to_string(),
        "source edge".to_string(),
        "carrier".to_string(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        "upstream".to_string(),
        PlanarBooleanSplitDecisionReason::SplitPhaseDenied("denial".to_string()),
        Some("denial".to_string()),
    );
}
