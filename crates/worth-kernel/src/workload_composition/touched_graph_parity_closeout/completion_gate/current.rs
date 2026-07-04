use super::error::{
    WorthTouchedGraphRoadmapCompletionGateError, WorthTouchedGraphRoadmapCompletionGateErrorKind,
};
use super::gate::{
    RoadmapCompletionFirewallCertification, WorthTouchedGraphRoadmapCompletionGate,
};
use super::validation::validate_roadmap_completion_gate;
use crate::workload_composition::touched_graph_parity_closeout::current_touched_graph_parity_closeout_authorities;

pub fn current_worth_touched_graph_roadmap_completion_gate(
) -> Result<WorthTouchedGraphRoadmapCompletionGate, WorthTouchedGraphRoadmapCompletionGateError> {
    let authorities = current_touched_graph_parity_closeout_authorities().map_err(|error| {
        WorthTouchedGraphRoadmapCompletionGateError::new(
            WorthTouchedGraphRoadmapCompletionGateErrorKind::CurrentCloseoutMatrixUnavailable,
            error.detail(),
        )
    })?;

    let gate = WorthTouchedGraphRoadmapCompletionGate::candidate(
        authorities.closeout_matrix().clone(),
        authorities.readiness_handoff().clone(),
        authorities.representative_path().clone(),
        authorities.public_closeout().clone(),
        RoadmapCompletionFirewallCertification::new(
            authorities
                .source_firewall_closeout()
                .source_firewall_report_digest(),
            authorities.source_firewall_closeout().deletion_closeout_digest(),
            authorities
                .source_firewall_closeout()
                .covered_forbidden_surfaces()
                .len(),
            authorities.source_firewall_closeout().closeout_digest(),
        ),
        authorities.live_coverage_ledger().clone(),
    );
    validate_roadmap_completion_gate(&gate)?;
    Ok(gate.mark_complete())
}
