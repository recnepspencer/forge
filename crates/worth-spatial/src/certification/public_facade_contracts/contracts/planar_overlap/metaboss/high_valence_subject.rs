use worth_kernel::workload_composition::{
    WorkloadCatalog, WorkloadCatalogError, WorkloadCatalogRecipe, WorkloadCatalogSupportPosture,
    WorkloadTopologyBreadth,
};
use worth_spatial::facade::high_valence_singularity::{
    HighValenceEvidenceIntegrity, HighValencePredicateCertification,
    HighValenceRebuildMotionCompatibility, HighValenceSingularityPolicy,
    HighValenceSingularityReceipt, HighValenceSingularityWorkload,
    HighValenceSingularityWorkloadError,
};
use worth_spatial::facade::user_response::{
    WorthUserOutcome, WorthUserResponseSource, WorthUserResponseWorkload,
};
use worth_spatial::facade::workload_vocabulary::WorkloadEvidenceStage;

pub(crate) struct PlatformHighValenceSubject {
    pub(crate) receipt: HighValenceSingularityReceipt,
    pub(crate) user_outcome: WorthUserOutcome,
}

pub(crate) fn certify_platform_high_valence_singularity(
    world: &'static str,
) -> PlatformHighValenceSubject {
    certify_platform_high_valence_singularity_with_valence(world, None)
}

pub(crate) fn certify_platform_high_valence_singularity_with_explicit_valence(
    world: &'static str,
    valence: usize,
) -> PlatformHighValenceSubject {
    certify_platform_high_valence_singularity_with_valence(world, Some(valence))
}

fn certify_platform_high_valence_singularity_with_valence(
    world: &'static str,
    valence: Option<usize>,
) -> PlatformHighValenceSubject {
    let recipe = valence.map_or_else(
        || high_valence_recipe(world),
        |valence| {
            high_valence_recipe(world)
                .with_topology_breadth(WorkloadTopologyBreadth::HighValenceVertex { valence })
        },
    );
    let built = recipe
        .build()
        .expect("platform high-valence workload should build");
    let receipt = HighValenceSingularityWorkload::from_platform_evidence(
        built.workload().evidence_ledger(),
        built.topology_neighborhood(),
    )
    .certify()
    .expect("platform high-valence singularity should certify");
    let user_outcome = user_outcome_from_high_valence_receipt(&receipt);

    PlatformHighValenceSubject {
        receipt,
        user_outcome,
    }
}

pub(crate) fn high_valence_unsupported_explicit_valence_outcome(
    world: &'static str,
    valence: usize,
) -> WorthUserOutcome {
    let unsupported_recipe = high_valence_recipe(world)
        .with_topology_breadth(WorkloadTopologyBreadth::HighValenceVertex { valence });
    let support = unsupported_recipe
        .inspect_support()
        .expect("unsupported high-valence support should be query-backed");

    assert_eq!(
        support.posture(),
        WorkloadCatalogSupportPosture::Unsupported
    );
    assert_eq!(
        support.human_reason(),
        format!(
            "high valence vertex workload recipe supports valence 3 through 128 today; valence {valence} needs an explicit widening phase"
        )
    );
    match unsupported_recipe
        .build()
        .expect_err("unsupported high-valence breadth must deny before topology construction")
    {
        WorkloadCatalogError::UnsupportedRecipe { recipe, reason } => {
            assert_eq!(recipe.human_name(), "high valence vertex workload recipe");
            assert!(reason.contains(&format!("valence {valence}")));
        }
        other => panic!("expected unsupported high-valence catalog denial, got {other:?}"),
    }

    user_outcome_from_high_valence_error(HighValenceSingularityWorkloadError::UnsupportedValence {
        valence,
    })
}

pub(crate) fn high_valence_policy_required_outcome(world: &'static str) -> WorthUserOutcome {
    high_valence_error_outcome(world, |workload| {
        workload.requiring_singularity_policy(HighValenceSingularityPolicy::RequiresUserDecision)
    })
}

pub(crate) fn high_valence_predicate_uncertain_outcome(world: &'static str) -> WorthUserOutcome {
    high_valence_error_outcome(world, |workload| {
        workload.requiring_predicate_certification(HighValencePredicateCertification::Uncertain)
    })
}

pub(crate) fn high_valence_missing_neighborhood_outcome(world: &'static str) -> WorthUserOutcome {
    let built = high_valence_recipe(world)
        .build()
        .expect("platform high-valence workload should build");
    let error = HighValenceSingularityWorkload::from_platform_evidence(
        built.workload().evidence_ledger(),
        None,
    )
    .certify()
    .expect_err("missing topology neighborhood must deny high-valence certification");
    user_outcome_from_high_valence_error(error)
}

pub(crate) fn high_valence_unsupported_valence_outcome(world: &'static str) -> WorthUserOutcome {
    high_valence_unsupported_explicit_valence_outcome(world, 129)
}

pub(crate) fn high_valence_rebuild_motion_break_outcome(world: &'static str) -> WorthUserOutcome {
    high_valence_error_outcome(world, |workload| {
        workload.requiring_rebuild_motion_compatibility(
                HighValenceRebuildMotionCompatibility::Incompatible {
                    reason: "high-valence rebuild motion must match retained neighborhood posture before correspondence".to_string(),
                },
            )
    })
}

pub(crate) fn high_valence_integrity_mismatch_outcome(world: &'static str) -> WorthUserOutcome {
    high_valence_error_outcome(world, |workload| {
        workload.requiring_evidence_integrity(
            HighValenceEvidenceIntegrity::MismatchedProjectedNeighborhood {
                stage: WorkloadEvidenceStage::Projection,
            },
        )
    })
}

fn high_valence_error_outcome<F>(world: &'static str, configure: F) -> WorthUserOutcome
where
    F: for<'a> FnOnce(HighValenceSingularityWorkload<'a>) -> HighValenceSingularityWorkload<'a>,
{
    let built = high_valence_recipe(world)
        .build()
        .expect("platform high-valence workload should build");
    let error = configure(HighValenceSingularityWorkload::from_platform_evidence(
        built.workload().evidence_ledger(),
        built.topology_neighborhood(),
    ))
    .certify()
    .expect_err("configured high-valence branch must deny");
    user_outcome_from_high_valence_error(error)
}

fn high_valence_recipe(world: &'static str) -> WorkloadCatalogRecipe {
    WorkloadCatalog::high_valence_vertex()
        .declared(format!("MB-M6-2 platform high-valence singularity {world}"))
}

fn user_outcome_from_high_valence_receipt(
    receipt: &HighValenceSingularityReceipt,
) -> WorthUserOutcome {
    WorthUserResponseWorkload::from_source(WorthUserResponseSource::from_high_valence_singularity(
        receipt,
    ))
    .declared("explain high-valence singularity outcome")
    .respond()
    .expect("high-valence user response should certify")
    .outcome()
    .clone()
}

fn user_outcome_from_high_valence_error(
    error: HighValenceSingularityWorkloadError,
) -> WorthUserOutcome {
    WorthUserResponseWorkload::from_source(
        WorthUserResponseSource::from_high_valence_singularity_error(error),
    )
    .declared("explain high-valence singularity denial")
    .respond()
    .expect("high-valence denial response should certify")
    .outcome()
    .clone()
}
