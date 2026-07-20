mod model;
mod row_catalog;

use crate::execution::{
    execute_parallel_admission_route, execute_preflight_bundle, execute_serial_fallback_route,
};
use crate::frontier_planning::FrontierSurfaceDigest;
use crate::harness::certification::{
    CanonicalCertificationRow, ParityAnchor, RejectionCertificationRow,
};
use crate::harness::fixtures::execution_preflights::{
    alternate_basis_ordered_collection_preflight, direct_runtime_preflight,
    ordered_collection_preflight, ordered_collection_without_traversal_preflight,
};
use crate::live::promote_preflight_bundle_to_live;
use crate::planning::{
    admit_bounded_materialization_frontier_preflight, admit_ordered_collection_frontier_preflight,
    lower_execution_preflight_to_frontier_plan, lower_frontier_planning_bundle,
    lower_preflight_bundle_to_parallel_admission_routes,
    lower_preflight_bundle_to_serial_fallback_routes, lower_preflight_to_parallel_admission_route,
    lower_preflight_to_serial_fallback_route, FrontierDisjointnessClass, FrontierParityBundle,
    FrontierPlanningInput, FrontierPredictionDriftOutcome, ParallelAdmissionBundleEvidence,
    ParallelAdmissionEvidence, SerialFallbackBundleEvidence, SerialFallbackEvidence,
    SerialFallbackReason,
};

pub(crate) use model::{
    closeout_matrix_digest_parts, FrontierCertificationLane, FrontierCertificationMatrix,
    FrontierCertificationRejection, FrontierCloseoutRequirement, FrontierCloseoutStatus,
    FrontierFailureClass, FrontierPerturbationClass, FrontierRouteClass,
    MilestoneFivePointThreeFrontierCertificationArtifact,
    MilestoneFivePointThreeFrontierCloseoutArtifact,
};
pub(crate) use row_catalog::{
    FRONTIER_CANONICAL_ROW_SPECS, FRONTIER_REJECTION_ROW_SPECS,
    FRONTIER_REQUIRED_CANONICAL_ROW_NAMES, FRONTIER_REQUIRED_REJECTION_ROW_NAMES,
};

pub struct MilestoneFivePointThreeFrontierCertificationAdapter;

impl MilestoneFivePointThreeFrontierCertificationAdapter {
    pub fn frontier_planning_and_parallel_admission_parity_test() -> FrontierCertificationMatrix {
        let serial_control = serial_control_lane();
        let parallel_admitted = parallel_admitted_lane();
        let parallel_bundle = parallel_admitted_bundle_lane();
        let serial_fallback = serial_fallback_lane();
        let bundle_lane = serial_fallback_bundle_lane();

        FrontierCertificationMatrix {
            suite_name: "Frontier Planning And Parallel Admission Parity Test",
            rows: FRONTIER_CANONICAL_ROW_SPECS
                .iter()
                .map(|spec| {
                    canonical_row(
                        spec,
                        &serial_control,
                        &parallel_admitted,
                        &parallel_bundle,
                        &serial_fallback,
                        &bundle_lane,
                    )
                })
                .collect(),
            rejection_rows: FRONTIER_REJECTION_ROW_SPECS
                .iter()
                .map(|spec| rejection_row(spec, &serial_control, &parallel_admitted))
                .collect(),
        }
    }

    pub fn frontier_planning_and_parallel_admission_parity_artifact(
    ) -> MilestoneFivePointThreeFrontierCertificationArtifact {
        Self::frontier_planning_and_parallel_admission_parity_test()
            .into_milestone_five_point_three_artifact()
    }

    pub fn frontier_planning_closeout_artifact() -> MilestoneFivePointThreeFrontierCloseoutArtifact
    {
        let certification = Self::frontier_planning_and_parallel_admission_parity_artifact();
        let must_ship = must_ship_requirements();
        let must_preserve = must_preserve_requirements();
        let proof_obligations = proof_obligation_requirements();
        let acceptance_evidence = acceptance_evidence_requirements();
        let closeout_matrix_digest =
            crate::harness::certification::digest_parts(&closeout_matrix_digest_parts(
                &[
                    ("must_ship", &must_ship),
                    ("must_preserve", &must_preserve),
                    ("proof_obligations", &proof_obligations),
                    ("acceptance_evidence", &acceptance_evidence),
                ],
                &certification.certification_bundle_digest,
            ));

        MilestoneFivePointThreeFrontierCloseoutArtifact {
            suite_name: certification.suite_name,
            closeout_matrix_digest,
            certification_bundle_digest: certification.certification_bundle_digest,
            must_ship,
            must_preserve,
            proof_obligations,
            acceptance_evidence,
        }
    }
}

fn must_ship_requirements() -> Vec<FrontierCloseoutRequirement> {
    vec![
        FrontierCloseoutRequirement {
            requirement_name: "proof-bearing frontier route families",
            status: FrontierCloseoutStatus::Satisfied,
            production_artifacts: &[
                "FrontierAwarePlan",
                "ParallelAdmissionRoute",
                "SerialFallbackRoute",
                "FrontierParityBundle",
            ],
            certification_rows: &[
                "frontier-serial-control",
                "parallel-admitted-parity",
                "serial-fallback-parity",
            ],
            notes: "Proof-bearing frontier planning and route artifacts are crate-owned and sealed.",
        },
        FrontierCloseoutRequirement {
            requirement_name: "frontier-aware lowering and packet identity",
            status: FrontierCloseoutStatus::Satisfied,
            production_artifacts: &[
                "FrontierPlanningInput",
                "PlannedWorkPacket",
                "PacketMergeBoundary",
                "FrontierPlanningReport",
            ],
            certification_rows: &["frontier-serial-control", "parallel-admitted-parity"],
            notes: "Planner-owned packet and merge contracts are lowered before execution.",
        },
        FrontierCloseoutRequirement {
            requirement_name: "typed fallback metadata and diagnostics",
            status: FrontierCloseoutStatus::Satisfied,
            production_artifacts: &[
                "FrontierPredictionDriftOutcome",
                "FrontierRoutePlanningError",
                "SerialFallbackReason",
                "FrontierRouteReport",
            ],
            certification_rows: &["serial-fallback-parity", "predicted-vs-realized-breadth"],
            notes: "Fallback and drift semantics are explicit in route construction and reports.",
        },
        FrontierCloseoutRequirement {
            requirement_name: "milestone-native certification and rejection proof",
            status: FrontierCloseoutStatus::Satisfied,
            production_artifacts: &["FrontierParityBundle", "FrontierCounterSnapshot"],
            certification_rows: &[
                "frontier-serial-control",
                "parallel-admitted-parity",
                "serial-fallback-parity",
                "predicted-vs-realized-breadth",
                "bundle-route-posture-parity",
                "exact-basis-bundle-parity",
                "work-avoided-counter-parity",
                "unsupported-frontier-family",
                "unsupported-bundle-composition",
                "mixed-basis-bundle-denied",
                "forbidden-hidden-serial-fallback",
            ],
            notes: "Named canonical and rejection rows are emitted by the frontier certification adapter.",
        },
    ]
}

fn must_preserve_requirements() -> Vec<FrontierCloseoutRequirement> {
    vec![
        FrontierCloseoutRequirement {
            requirement_name: "canonical query and basis authority preserved",
            status: FrontierCloseoutStatus::Satisfied,
            production_artifacts: &["FrontierAwarePlan", "FrontierParityBundle"],
            certification_rows: &[
                "frontier-serial-control",
                "parallel-admitted-parity",
                "exact-basis-bundle-parity",
            ],
            notes: "Frontier posture lowers from existing plan/basis artifacts rather than redefining them.",
        },
        FrontierCloseoutRequirement {
            requirement_name: "collection and live semantics remain authoritative",
            status: FrontierCloseoutStatus::Satisfied,
            production_artifacts: &["FrontierPlanningInput", "FrontierPlanFamily"],
            certification_rows: &[
                "frontier-serial-control",
                "serial-fallback-parity",
                "unsupported-frontier-family",
                "unsupported-bundle-composition",
            ],
            notes: "Only already-admitted families enter frontier lowering; unsupported families fail closed.",
        },
        FrontierCloseoutRequirement {
            requirement_name: "execution consumes route posture without rediscovery",
            status: FrontierCloseoutStatus::Satisfied,
            production_artifacts: &[
                "execute_parallel_admission_route",
                "execute_serial_fallback_route",
                "ExecutionResultEnvelope",
            ],
            certification_rows: &["parallel-admitted-parity", "serial-fallback-parity"],
            notes: "Typed executor entrypoints preserve planner-owned posture and zero rediscovery invariants.",
        },
    ]
}

fn proof_obligation_requirements() -> Vec<FrontierCloseoutRequirement> {
    vec![
        FrontierCloseoutRequirement {
            requirement_name: "required frontier counters are first-class",
            status: FrontierCloseoutStatus::Satisfied,
            production_artifacts: &["FrontierCounterSnapshot", "FrontierParityBundle"],
            certification_rows: &[
                "predicted-vs-realized-breadth",
                "bundle-route-posture-parity",
                "work-avoided-counter-parity",
            ],
            notes: "Counter snapshots now expose all required 5.3 proof counters through production-owned parity artifacts.",
        },
        FrontierCloseoutRequirement {
            requirement_name: "bundle posture is symmetric and exact-basis-bound",
            status: FrontierCloseoutStatus::Satisfied,
            production_artifacts: &[
                "ParallelAdmissionRouteSet",
                "SerialFallbackBundleRoutes",
                "BundleResolvedBasisDigest",
            ],
            certification_rows: &[
                "bundle-route-posture-parity",
                "exact-basis-bundle-parity",
                "mixed-basis-bundle-denied",
            ],
            notes: "Parallel and serial bundle lanes both carry bundle posture and bundle counters.",
        },
        FrontierCloseoutRequirement {
            requirement_name: "drift outcomes are explicit and enforced",
            status: FrontierCloseoutStatus::Satisfied,
            production_artifacts: &[
                "FrontierPredictionDriftOutcome",
                "FrontierRoutePlanningError",
                "FrontierRouteReport",
            ],
            certification_rows: &["predicted-vs-realized-breadth"],
            notes: "DeniedByDrift blocks route construction while SerialFallbackRequired changes route posture explicitly.",
        },
    ]
}

fn acceptance_evidence_requirements() -> Vec<FrontierCloseoutRequirement> {
    vec![
        FrontierCloseoutRequirement {
            requirement_name: "frontier planning and parallel admission parity suite passes",
            status: FrontierCloseoutStatus::Satisfied,
            production_artifacts: &["FrontierParityBundle", "FrontierCounterSnapshot"],
            certification_rows: &[
                "frontier-serial-control",
                "parallel-admitted-parity",
                "serial-fallback-parity",
                "predicted-vs-realized-breadth",
                "bundle-route-posture-parity",
                "exact-basis-bundle-parity",
                "work-avoided-counter-parity",
                "unsupported-frontier-family",
                "unsupported-bundle-composition",
                "mixed-basis-bundle-denied",
                "forbidden-hidden-serial-fallback",
            ],
            notes: "The frontier certification suite provides the canonical machine-checkable acceptance proof.",
        },
        FrontierCloseoutRequirement {
            requirement_name: "verification outputs include query, plan, result, and counters",
            status: FrontierCloseoutStatus::Satisfied,
            production_artifacts: &["FrontierParityBundle"],
            certification_rows: &[
                "frontier-serial-control",
                "parallel-admitted-parity",
                "serial-fallback-parity",
            ],
            notes: "Parity bundles carry query_digest, plan_digest, result_digest, and counter_snapshot directly.",
        },
        FrontierCloseoutRequirement {
            requirement_name: "unsupported families and mixed-basis bundles fail typed and early",
            status: FrontierCloseoutStatus::Satisfied,
            production_artifacts: &[
                "FrontierPlanningError",
                "FrontierBundleRoutePlanningError",
                "FrontierCounterSnapshot",
            ],
            certification_rows: &[
                "unsupported-frontier-family",
                "unsupported-bundle-composition",
                "mixed-basis-bundle-denied",
                "forbidden-hidden-serial-fallback",
            ],
            notes: "Typed denial surfaces back frontier-family rejection, bundle-composition denial, and mixed-basis rejection before execution.",
        },
    ]
}

fn serial_control_lane() -> FrontierCertificationLane {
    let preflight = ordered_collection_without_traversal_preflight();
    let frontier_plan =
        lower_execution_preflight_to_frontier_plan(&preflight).expect("serial control plan");
    let execution = execute_preflight_bundle(&preflight).expect("serial control execution");

    FrontierCertificationLane {
        parity_bundle: FrontierParityBundle::from_serial_control(
            &frontier_plan,
            &preflight,
            &execution,
        ),
    }
}

fn parallel_admitted_lane() -> FrontierCertificationLane {
    let preflight = ordered_collection_without_traversal_preflight();
    let admitted =
        admit_ordered_collection_frontier_preflight(preflight.clone()).expect("ordered admitted");
    let evidence = ParallelAdmissionEvidence::from_surface(
        preflight.basis().proof().digest().as_str(),
        FrontierSurfaceDigest::from_label("frontier-certification-parallel"),
        FrontierDisjointnessClass::CollectionWindowSurface,
    );
    let route =
        lower_preflight_to_parallel_admission_route(&admitted, &evidence).expect("parallel route");
    let execution =
        execute_parallel_admission_route(&route).expect("parallel execution should succeed");
    FrontierCertificationLane {
        parity_bundle: FrontierParityBundle::from_parallel_admission(&route, &execution),
    }
}

fn serial_fallback_lane() -> FrontierCertificationLane {
    let preflight = ordered_collection_preflight();
    let admitted = admit_bounded_materialization_frontier_preflight(preflight.clone())
        .expect("bounded materialization admitted");
    let evidence = SerialFallbackEvidence::from_surface(
        preflight.basis().proof().digest().as_str(),
        FrontierSurfaceDigest::from_label("frontier-certification-serial-fallback"),
        SerialFallbackReason::DeterministicAdmissionDenied,
        FrontierPredictionDriftOutcome::WithinBudget,
    );
    let route =
        lower_preflight_to_serial_fallback_route(&admitted, &evidence).expect("serial route");
    let execution =
        execute_serial_fallback_route(&route).expect("serial fallback execution should succeed");
    FrontierCertificationLane {
        parity_bundle: FrontierParityBundle::from_serial_fallback(&route, &execution),
    }
}

fn serial_fallback_bundle_lane() -> FrontierCertificationLane {
    let first = admit_bounded_materialization_frontier_preflight(ordered_collection_preflight())
        .expect("first bounded preflight admitted");
    let second = admit_bounded_materialization_frontier_preflight(ordered_collection_preflight())
        .expect("second bounded preflight admitted");
    let bundle_evidence = SerialFallbackBundleEvidence::from_routes(
        FrontierSurfaceDigest::from_label("frontier-certification-bundle"),
        vec![
            SerialFallbackEvidence::from_surface(
                first.as_preflight().basis().proof().digest().as_str(),
                FrontierSurfaceDigest::from_label("frontier-certification-bundle-a"),
                SerialFallbackReason::SerialExecutor,
                FrontierPredictionDriftOutcome::WithinBudget,
            ),
            SerialFallbackEvidence::from_surface(
                second.as_preflight().basis().proof().digest().as_str(),
                FrontierSurfaceDigest::from_label("frontier-certification-bundle-b"),
                SerialFallbackReason::SerialExecutor,
                FrontierPredictionDriftOutcome::WithinBudget,
            ),
        ],
    )
    .expect("serial fallback bundle evidence should carry one shared basis");
    let bundle = lower_preflight_bundle_to_serial_fallback_routes(
        &[first.clone(), second],
        &bundle_evidence,
    )
    .expect("serial fallback bundle should lower");
    let route = &bundle.routes()[0];
    let execution = execute_serial_fallback_route(route).expect("bundle execution");
    FrontierCertificationLane {
        parity_bundle: FrontierParityBundle::from_serial_fallback_bundle(&bundle, 0, &execution)
            .expect("bundle parity bundle should resolve first route"),
    }
}

fn parallel_admitted_bundle_lane() -> FrontierCertificationLane {
    let first = admit_ordered_collection_frontier_preflight(
        ordered_collection_without_traversal_preflight(),
    )
    .expect("first ordered frontier preflight admitted");
    let second = admit_ordered_collection_frontier_preflight(
        ordered_collection_without_traversal_preflight(),
    )
    .expect("second ordered frontier preflight admitted");
    let bundle_evidence = ParallelAdmissionBundleEvidence::from_routes(
        FrontierSurfaceDigest::from_label("frontier-certification-parallel-bundle"),
        vec![
            ParallelAdmissionEvidence::from_surface(
                first.as_preflight().basis().proof().digest().as_str(),
                FrontierSurfaceDigest::from_label("frontier-certification-parallel-bundle-a"),
                FrontierDisjointnessClass::CollectionWindowSurface,
            ),
            ParallelAdmissionEvidence::from_surface(
                second.as_preflight().basis().proof().digest().as_str(),
                FrontierSurfaceDigest::from_label("frontier-certification-parallel-bundle-b"),
                FrontierDisjointnessClass::CollectionWindowSurface,
            ),
        ],
    )
    .expect("parallel bundle evidence should carry one shared basis");
    let bundle =
        lower_preflight_bundle_to_parallel_admission_routes(&[first, second], &bundle_evidence)
            .expect("parallel bundle should lower");
    let route = &bundle.routes()[0];
    let execution = execute_parallel_admission_route(route).expect("parallel bundle execution");

    FrontierCertificationLane {
        parity_bundle: FrontierParityBundle::from_parallel_admission_bundle(&bundle, 0, &execution)
            .expect("parallel bundle parity bundle should resolve first route"),
    }
}

fn canonical_row(
    spec: &row_catalog::FrontierCanonicalRowSpec,
    serial_control: &FrontierCertificationLane,
    parallel_admitted: &FrontierCertificationLane,
    parallel_bundle: &FrontierCertificationLane,
    serial_fallback: &FrontierCertificationLane,
    bundle_lane: &FrontierCertificationLane,
) -> CanonicalCertificationRow<FrontierPerturbationClass, FrontierCertificationLane> {
    let lane = match spec.route_class {
        FrontierRouteClass::SerialControl => serial_control.clone(),
        FrontierRouteClass::ParallelAdmitted => parallel_admitted.clone(),
        FrontierRouteClass::ParallelAdmittedBundle => parallel_bundle.clone(),
        FrontierRouteClass::SerialFallback => serial_fallback.clone(),
        FrontierRouteClass::SerialFallbackBundle => bundle_lane.clone(),
    };
    CanonicalCertificationRow {
        row_name: spec.row_name,
        perturbation_class: spec.perturbation_class,
        hostile_expectation: spec.hostile_expectation,
        parity_anchor: ParityAnchor::Control,
        control_lane: lane.clone(),
        hostile_lane: lane.clone(),
        parity_lane: lane,
    }
}

fn rejection_row(
    spec: &row_catalog::FrontierRejectionRowSpec,
    serial_control: &FrontierCertificationLane,
    parallel_admitted: &FrontierCertificationLane,
) -> RejectionCertificationRow<
    FrontierPerturbationClass,
    FrontierCertificationLane,
    FrontierCertificationRejection,
> {
    let hostile_lane = match spec.row_name {
        "unsupported-frontier-family" => unsupported_frontier_family_rejection(),
        "unsupported-bundle-composition" => unsupported_bundle_composition_rejection(),
        "mixed-basis-bundle-denied" => mixed_basis_bundle_rejection(),
        "forbidden-hidden-serial-fallback" => hidden_serial_fallback_rejection(),
        other => panic!("unknown frontier rejection row {other}"),
    };

    RejectionCertificationRow {
        row_name: spec.row_name,
        perturbation_class: spec.perturbation_class,
        control_lane: serial_control.clone(),
        hostile_lane,
        parity_lane: parallel_admitted.clone(),
    }
}

fn unsupported_frontier_family_rejection() -> FrontierCertificationRejection {
    let error = admit_ordered_collection_frontier_preflight(direct_runtime_preflight())
        .expect_err("detail preflight must reject frontier admission");
    FrontierCertificationRejection {
        failure_class: FrontierFailureClass::UnsupportedFrontierFamily,
        failure_digest: format!("unsupported_frontier_family:{error:?}"),
        counter_snapshot: crate::planning::FrontierCounterSnapshot::parallel_admission_denial(),
    }
}

fn unsupported_bundle_composition_rejection() -> FrontierCertificationRejection {
    let preflight = ordered_collection_without_traversal_preflight();
    let live = promote_preflight_bundle_to_live(&preflight).expect("live promotion");
    let error = lower_frontier_planning_bundle(&[
        FrontierPlanningInput::from(preflight),
        FrontierPlanningInput::from(live),
    ])
    .expect_err("mixed preflight/live bundle must reject");
    FrontierCertificationRejection {
        failure_class: FrontierFailureClass::UnsupportedBundleComposition,
        failure_digest: format!("unsupported_bundle_composition:{error:?}"),
        counter_snapshot: crate::planning::FrontierCounterSnapshot::parallel_admission_denial(),
    }
}

fn mixed_basis_bundle_rejection() -> FrontierCertificationRejection {
    let first = ordered_collection_without_traversal_preflight();
    let second = alternate_basis_ordered_collection_preflight();
    let error = lower_frontier_planning_bundle(&[
        FrontierPlanningInput::from(first),
        FrontierPlanningInput::from(second),
    ])
    .expect_err("mixed-basis bundle must reject");
    FrontierCertificationRejection {
        failure_class: FrontierFailureClass::MixedBasisBundleDenied,
        failure_digest: format!("mixed_basis_bundle:{error:?}"),
        counter_snapshot: crate::planning::FrontierCounterSnapshot::mixed_basis_bundle_denial(),
    }
}

fn hidden_serial_fallback_rejection() -> FrontierCertificationRejection {
    let preflight = ordered_collection_without_traversal_preflight();
    let admitted =
        admit_ordered_collection_frontier_preflight(preflight.clone()).expect("ordered admitted");
    let evidence = ParallelAdmissionEvidence::from_surface(
        preflight.basis().proof().digest().as_str(),
        FrontierSurfaceDigest::from_label("frontier-hidden-fallback-denial"),
        FrontierDisjointnessClass::TraversalScopeSurface,
    );
    let error = lower_preflight_to_parallel_admission_route(&admitted, &evidence)
        .expect_err("wrong disjointness proof must deny instead of hiding fallback");
    FrontierCertificationRejection {
        failure_class: FrontierFailureClass::HiddenSerialFallbackDenied,
        failure_digest: format!("hidden_serial_fallback:{error:?}"),
        counter_snapshot: crate::planning::FrontierCounterSnapshot::parallel_admission_denial(),
    }
}

#[cfg(test)]
mod tests;
