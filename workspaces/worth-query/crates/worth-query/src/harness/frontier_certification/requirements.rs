use super::{FrontierCloseoutRequirement, FrontierCloseoutStatus};

pub(super) fn must_ship_requirements() -> Vec<FrontierCloseoutRequirement> {
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

pub(super) fn must_preserve_requirements() -> Vec<FrontierCloseoutRequirement> {
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

pub(super) fn proof_obligation_requirements() -> Vec<FrontierCloseoutRequirement> {
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

pub(super) fn acceptance_evidence_requirements() -> Vec<FrontierCloseoutRequirement> {
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
