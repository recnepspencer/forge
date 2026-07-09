use worth_query::facade::{
    WORTHQueryBindingAspectConflict, WORTHQueryBindingMissingRequiredAspect,
    WORTHQueryBindingRebindRequired, WORTHQueryBindingStale,
};
use hadwiger_research::facade::*;

#[test]
fn query_aspect_contract_mapping_uses_exact_hadwiger_paths() {
    let unit_distance =
        query_aspect_contract_for_hadwiger_kind(HadwigerAspectKind::UnitDistanceEmbedding);
    let lower_bound =
        query_aspect_contract_for_hadwiger_kind(HadwigerAspectKind::LowerBoundWitness);
    let advisory = query_aspect_contract_for_hadwiger_kind(HadwigerAspectKind::AIAdvisory);

    assert_eq!(
        unit_distance.required(),
        &["hadwiger.embedding.unit_distance".to_string()]
    );
    assert_eq!(
        lower_bound.required(),
        &["hadwiger.lower_bound.witness".to_string()]
    );
    assert_eq!(
        lower_bound.preserved(),
        &[
            "hadwiger.colorability.not_k_colorable".to_string(),
            "hadwiger.embedding.unit_distance".to_string(),
        ]
    );
    assert_eq!(
        lower_bound.incompatible(),
        &["hadwiger.ai.advisory".to_string()]
    );
    assert_eq!(advisory.required(), &[] as &[String]);
    assert_eq!(advisory.published(), &["hadwiger.ai.advisory".to_string()]);
}

#[test]
fn all_hadwiger_aspect_kinds_have_stable_query_contract_paths() {
    let exact_paths = [
        (
            HadwigerAspectKind::AbstractGraphStructure,
            "hadwiger.graph.abstract_structure",
        ),
        (
            HadwigerAspectKind::GraphVersionShape,
            "hadwiger.graph.version_shape",
        ),
        (
            HadwigerAspectKind::EmbeddingCandidate,
            "hadwiger.embedding.candidate",
        ),
        (
            HadwigerAspectKind::UnitDistanceEmbedding,
            "hadwiger.embedding.unit_distance",
        ),
        (
            HadwigerAspectKind::KColorabilityEncoding,
            "hadwiger.colorability.encoding",
        ),
        (
            HadwigerAspectKind::SolverRunEvidence,
            "hadwiger.colorability.solver_run",
        ),
        (
            HadwigerAspectKind::NotKColorable,
            "hadwiger.colorability.not_k_colorable",
        ),
        (
            HadwigerAspectKind::UnsatCore,
            "hadwiger.colorability.unsat_core",
        ),
        (
            HadwigerAspectKind::GadgetContract,
            "hadwiger.gadget.contract",
        ),
        (
            HadwigerAspectKind::ReductionTrace,
            "hadwiger.reduction.trace",
        ),
        (
            HadwigerAspectKind::GraphComposition,
            "hadwiger.graph.composition",
        ),
        (
            HadwigerAspectKind::LowerBoundWitness,
            "hadwiger.lower_bound.witness",
        ),
        (HadwigerAspectKind::AIAdvisory, "hadwiger.ai.advisory"),
        (
            HadwigerAspectKind::FailureEvidence,
            "hadwiger.failure.evidence",
        ),
    ];

    for (aspect_kind, expected_path) in exact_paths {
        let contract = query_aspect_contract_for_hadwiger_kind(aspect_kind);
        assert!(
            contract.required().contains(&expected_path.to_string())
                || contract.published().contains(&expected_path.to_string()),
            "{aspect_kind:?} should expose {expected_path}"
        );
    }
}

#[test]
fn query_coverage_and_publication_reflect_aspect_posture() {
    let admitted = query_aspect_coverage_for_hadwiger_posture(
        HadwigerAspectKind::GraphVersionShape,
        HadwigerAspectPosture::Admitted,
    );
    let stale = query_aspect_coverage_for_hadwiger_posture(
        HadwigerAspectKind::GraphVersionShape,
        HadwigerAspectPosture::Stale,
    );
    let conflict = query_aspect_coverage_for_hadwiger_posture(
        HadwigerAspectKind::GraphVersionShape,
        HadwigerAspectPosture::Conflict,
    );
    let publication =
        query_aspect_publication_for_hadwiger_kind(HadwigerAspectKind::GraphVersionShape);

    assert_eq!(
        admitted.present(),
        &["hadwiger.graph.version_shape".to_string()]
    );
    assert_eq!(
        stale.masked(),
        &["hadwiger.graph.version_shape".to_string()]
    );
    assert_eq!(
        conflict.conflicting(),
        &["hadwiger.graph.version_shape".to_string()]
    );
    assert_eq!(
        publication.present(),
        &["hadwiger.graph.version_shape".to_string()]
    );
}

#[test]
fn binding_stops_retain_typed_query_reasons() {
    let stale = AspectClosureStop::query_stale(
        HadwigerAspectKind::UnitDistanceEmbedding,
        WORTHQueryBindingStale::new("stale retained embedding"),
    );
    let rebind = AspectClosureStop::query_rebind_required(
        HadwigerAspectKind::UnitDistanceEmbedding,
        WORTHQueryBindingRebindRequired::new("rebind required"),
    );
    let missing = AspectClosureStop::query_missing_required_aspect(
        HadwigerAspectKind::UnitDistanceEmbedding,
        WORTHQueryBindingMissingRequiredAspect::new("unit-distance aspect missing"),
    );
    let conflict = AspectClosureStop::query_aspect_conflict(
        HadwigerAspectKind::UnitDistanceEmbedding,
        WORTHQueryBindingAspectConflict::new("unit-distance aspect conflicts"),
    );
    let local = AspectClosureStop::local(HadwigerAspectKind::FailureEvidence, "local stop");

    assert!(stale.is_query_owned());
    assert!(rebind.is_query_owned());
    assert!(missing.is_query_owned());
    assert!(conflict.is_query_owned());
    assert!(!local.is_query_owned());
    assert_eq!(missing.reason(), "unit-distance aspect missing");
    assert_eq!(
        missing.aspect_kind(),
        HadwigerAspectKind::UnitDistanceEmbedding
    );
}

#[test]
fn recompute_and_promotion_descriptors_do_not_admit_theorem_authority() {
    assert_eq!(
        HadwigerRecomputePolicy::ConservativeEscalationRequired.as_str(),
        "conservative_escalation_required"
    );
    let descriptor =
        HadwigerPromotionRuleDescriptor::lower_bound_witness_rule("phase-5-lower-bound").unwrap();

    assert_eq!(
        descriptor.target_aspect(),
        HadwigerAspectKind::LowerBoundWitness
    );
    assert_eq!(
        descriptor.required_aspects(),
        &[
            HadwigerAspectKind::UnitDistanceEmbedding,
            HadwigerAspectKind::NotKColorable,
        ]
    );
    assert!(!descriptor.admits_theorem_authority());
}
