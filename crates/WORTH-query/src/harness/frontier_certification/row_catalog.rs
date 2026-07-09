use crate::harness::certification::HostileExpectation;

use super::{FrontierFailureClass, FrontierPerturbationClass, FrontierRouteClass};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FrontierCanonicalRowSpec {
    pub row_name: &'static str,
    pub perturbation_class: FrontierPerturbationClass,
    pub hostile_expectation: HostileExpectation,
    pub route_class: FrontierRouteClass,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FrontierRejectionRowSpec {
    pub row_name: &'static str,
    pub perturbation_class: FrontierPerturbationClass,
    pub failure_class: FrontierFailureClass,
    pub compile_fail_case: Option<&'static str>,
}

pub const FRONTIER_CANONICAL_ROW_SPECS: &[FrontierCanonicalRowSpec] = &[
    FrontierCanonicalRowSpec {
        row_name: "frontier-serial-control",
        perturbation_class: FrontierPerturbationClass::SerialControlParity,
        hostile_expectation: HostileExpectation::EquivalentToControl,
        route_class: FrontierRouteClass::SerialControl,
    },
    FrontierCanonicalRowSpec {
        row_name: "parallel-admitted-parity",
        perturbation_class: FrontierPerturbationClass::ParallelAdmittedParity,
        hostile_expectation: HostileExpectation::EquivalentToControl,
        route_class: FrontierRouteClass::ParallelAdmitted,
    },
    FrontierCanonicalRowSpec {
        row_name: "serial-fallback-parity",
        perturbation_class: FrontierPerturbationClass::SerialFallbackParity,
        hostile_expectation: HostileExpectation::EquivalentToControl,
        route_class: FrontierRouteClass::SerialFallback,
    },
    FrontierCanonicalRowSpec {
        row_name: "predicted-vs-realized-breadth",
        perturbation_class: FrontierPerturbationClass::PredictedRealizedBreadth,
        hostile_expectation: HostileExpectation::EquivalentToControl,
        route_class: FrontierRouteClass::ParallelAdmitted,
    },
    FrontierCanonicalRowSpec {
        row_name: "bundle-route-posture-parity",
        perturbation_class: FrontierPerturbationClass::BundleRoutePostureParity,
        hostile_expectation: HostileExpectation::EquivalentToControl,
        route_class: FrontierRouteClass::ParallelAdmittedBundle,
    },
    FrontierCanonicalRowSpec {
        row_name: "exact-basis-bundle-parity",
        perturbation_class: FrontierPerturbationClass::ExactBasisBundleParity,
        hostile_expectation: HostileExpectation::EquivalentToControl,
        route_class: FrontierRouteClass::SerialFallbackBundle,
    },
    FrontierCanonicalRowSpec {
        row_name: "work-avoided-counter-parity",
        perturbation_class: FrontierPerturbationClass::WorkAvoidedCounterParity,
        hostile_expectation: HostileExpectation::EquivalentToControl,
        route_class: FrontierRouteClass::ParallelAdmitted,
    },
];

pub const FRONTIER_REJECTION_ROW_SPECS: &[FrontierRejectionRowSpec] = &[
    FrontierRejectionRowSpec {
        row_name: "unsupported-frontier-family",
        perturbation_class: FrontierPerturbationClass::UnsupportedFrontierFamilyRejection,
        failure_class: FrontierFailureClass::UnsupportedFrontierFamily,
        compile_fail_case: None,
    },
    FrontierRejectionRowSpec {
        row_name: "unsupported-bundle-composition",
        perturbation_class: FrontierPerturbationClass::UnsupportedBundleCompositionRejection,
        failure_class: FrontierFailureClass::UnsupportedBundleComposition,
        compile_fail_case: None,
    },
    FrontierRejectionRowSpec {
        row_name: "mixed-basis-bundle-denied",
        perturbation_class: FrontierPerturbationClass::MixedBasisBundleRejection,
        failure_class: FrontierFailureClass::MixedBasisBundleDenied,
        compile_fail_case: None,
    },
    FrontierRejectionRowSpec {
        row_name: "forbidden-executor-speculative-admission",
        perturbation_class: FrontierPerturbationClass::ExecutorSpeculativeAdmissionRejection,
        failure_class: FrontierFailureClass::CompileFail,
        compile_fail_case: Some("tests/ui/frontier_route_evidence_constructor_not_public.rs"),
    },
    FrontierRejectionRowSpec {
        row_name: "forbidden-hidden-serial-fallback",
        perturbation_class: FrontierPerturbationClass::HiddenSerialFallbackRejection,
        failure_class: FrontierFailureClass::HiddenSerialFallbackDenied,
        compile_fail_case: None,
    },
    FrontierRejectionRowSpec {
        row_name: "invalid-route-posture-override",
        perturbation_class: FrontierPerturbationClass::RoutePostureOverrideRejection,
        failure_class: FrontierFailureClass::CompileFail,
        compile_fail_case: Some("tests/ui/signal_frontier_adapter_route_minting_not_public.rs"),
    },
    FrontierRejectionRowSpec {
        row_name: "forbidden-serial-route-on-parallel-entrypoint",
        perturbation_class: FrontierPerturbationClass::SerialRouteOnParallelEntrypointRejection,
        failure_class: FrontierFailureClass::CompileFail,
        compile_fail_case: Some("tests/ui/serial_route_cannot_use_parallel_entrypoint.rs"),
    },
];

pub const FRONTIER_REQUIRED_CANONICAL_ROW_NAMES: &[&str] = &[
    "frontier-serial-control",
    "parallel-admitted-parity",
    "serial-fallback-parity",
    "predicted-vs-realized-breadth",
    "bundle-route-posture-parity",
    "exact-basis-bundle-parity",
    "work-avoided-counter-parity",
];

pub const FRONTIER_REQUIRED_REJECTION_ROW_NAMES: &[&str] = &[
    "unsupported-frontier-family",
    "unsupported-bundle-composition",
    "mixed-basis-bundle-denied",
    "forbidden-executor-speculative-admission",
    "forbidden-hidden-serial-fallback",
    "invalid-route-posture-override",
    "forbidden-serial-route-on-parallel-entrypoint",
];
