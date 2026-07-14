use hadwiger_research::facade::*;
use worth_query::facade::foundation::{
    AspectFieldKey, WorthQueryBindingAspectConflict, WorthQueryBindingMissingRequiredAspect,
    WorthQueryBindingRebindRequired, WorthQueryBindingStale,
};

fn aspect_field(path: &str) -> AspectFieldKey {
    let (aspect, field) = path
        .rsplit_once('.')
        .expect("test aspect paths include an aspect and field");
    AspectFieldKey::from_authoring_parts(aspect, field).expect("test aspect field should admit")
}

#[test]
fn query_aspect_contract_mapping_uses_exact_hadwiger_paths() {
    let unit_distance =
        query_aspect_contract_for_hadwiger_kind(HadwigerAspectKind::UnitDistanceEmbedding);
    let lower_bound =
        query_aspect_contract_for_hadwiger_kind(HadwigerAspectKind::LowerBoundWitness);
    let advisory = query_aspect_contract_for_hadwiger_kind(HadwigerAspectKind::AIAdvisory);

    assert_eq!(
        unit_distance.required(),
        &[aspect_field("hadwiger.embedding.unit_distance")]
    );
    assert_eq!(
        lower_bound.required(),
        &[aspect_field("hadwiger.lower_bound.witness")]
    );
    assert_eq!(
        lower_bound.preserved(),
        &[
            aspect_field("hadwiger.colorability.not_k_colorable"),
            aspect_field("hadwiger.embedding.unit_distance"),
        ]
    );
    assert_eq!(
        lower_bound.incompatible(),
        &[aspect_field("hadwiger.ai.advisory")]
    );
    assert_eq!(advisory.required(), &[] as &[AspectFieldKey]);
    assert_eq!(
        advisory.published(),
        &[aspect_field("hadwiger.ai.advisory")]
    );
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
        let expected_field = aspect_field(expected_path);
        assert!(
            contract.required().contains(&expected_field)
                || contract.published().contains(&expected_field),
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
        &[aspect_field("hadwiger.graph.version_shape")]
    );
    assert_eq!(
        stale.masked(),
        &[aspect_field("hadwiger.graph.version_shape")]
    );
    assert_eq!(
        conflict.conflicting(),
        &[aspect_field("hadwiger.graph.version_shape")]
    );
    assert_eq!(
        publication.present(),
        &[aspect_field("hadwiger.graph.version_shape")]
    );
}

#[test]
fn binding_stops_retain_typed_query_reasons() {
    let stale = AspectClosureStop::query_stale(
        HadwigerAspectKind::UnitDistanceEmbedding,
        WorthQueryBindingStale::new("stale retained embedding"),
    );
    let rebind = AspectClosureStop::query_rebind_required(
        HadwigerAspectKind::UnitDistanceEmbedding,
        WorthQueryBindingRebindRequired::new("rebind required"),
    );
    let missing = AspectClosureStop::query_missing_required_aspect(
        HadwigerAspectKind::UnitDistanceEmbedding,
        WorthQueryBindingMissingRequiredAspect::new("unit-distance aspect missing"),
    );
    let conflict = AspectClosureStop::query_aspect_conflict(
        HadwigerAspectKind::UnitDistanceEmbedding,
        WorthQueryBindingAspectConflict::new("unit-distance aspect conflicts"),
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
