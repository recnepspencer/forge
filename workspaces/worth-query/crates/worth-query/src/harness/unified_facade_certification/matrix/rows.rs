use super::super::lane::{UnifiedFacadeLane, UnifiedFacadePerturbationClass};
use super::super::row_catalog::UnifiedFacadeCanonicalRowSpec;
use crate::harness::certification::{CanonicalCertificationRow, ParityAnchor};

pub(super) fn canonical_row(
    spec: &UnifiedFacadeCanonicalRowSpec,
    runtime_query: &UnifiedFacadeLane,
    query_context: &UnifiedFacadeLane,
    identity_evolution: &UnifiedFacadeLane,
    runtime_live: &UnifiedFacadeLane,
    preview: &UnifiedFacadeLane,
    workflow: &UnifiedFacadeLane,
    historical: &UnifiedFacadeLane,
    config_section: &UnifiedFacadeLane,
    support_sync: &UnifiedFacadeLane,
    identity_support_sync: &UnifiedFacadeLane,
) -> CanonicalCertificationRow<UnifiedFacadePerturbationClass, UnifiedFacadeLane> {
    let (control_lane, hostile_lane) = match spec.row_name {
        "unified-query-read-capability" => (runtime_query.clone(), runtime_query.clone()),
        "unified-query-context-capability" => (query_context.clone(), query_context.clone()),
        "unified-identity-evolution-capability" => {
            (identity_evolution.clone(), identity_evolution.clone())
        }
        "unified-query-context-basis-result-bundle"
        | "unified-query-context-diff-result-bundle" => {
            (query_context.clone(), query_context.clone())
        }
        "unified-live-capability" => (runtime_live.clone(), runtime_live.clone()),
        "unified-preview-capability" => (preview.clone(), preview.clone()),
        "unified-workflow-capability" => (workflow.clone(), workflow.clone()),
        "unified-historical-capability" => (historical.clone(), historical.clone()),
        "unified-config-section-explicitness" => (runtime_query.clone(), config_section.clone()),
        "capability-support-metadata-sync" | "query-context-support-profile-sync" => {
            (support_sync.clone(), support_sync.clone())
        }
        "identity-evolution-support-profile-sync" => {
            (identity_support_sync.clone(), identity_support_sync.clone())
        }
        other => panic!("unexpected unified facade canonical row {other}"),
    };

    CanonicalCertificationRow {
        row_name: spec.row_name,
        perturbation_class: spec.perturbation_class,
        hostile_expectation: spec.hostile_expectation,
        parity_anchor: ParityAnchor::Control,
        control_lane: control_lane.clone(),
        hostile_lane,
        parity_lane: control_lane,
    }
}
