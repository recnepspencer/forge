use crate::frontier_planning::FrontierSurfaceDigest;
use crate::harness::fixtures::execution_preflights::{
    alternate_basis_ordered_collection_preflight, direct_runtime_preflight,
    ordered_collection_without_traversal_preflight,
};
use crate::live::promote_preflight_bundle_to_live;
use crate::planning::{
    admit_ordered_collection_frontier_preflight, lower_frontier_planning_bundle,
    lower_preflight_to_parallel_admission_route, FrontierDisjointnessClass, FrontierPlanningInput,
    ParallelAdmissionEvidence,
};

use super::{FrontierCertificationRejection, FrontierFailureClass};

pub(super) fn unsupported_frontier_family_rejection() -> FrontierCertificationRejection {
    let error = admit_ordered_collection_frontier_preflight(direct_runtime_preflight())
        .expect_err("detail preflight must reject frontier admission");
    FrontierCertificationRejection {
        failure_class: FrontierFailureClass::UnsupportedFrontierFamily,
        failure_digest: format!("unsupported_frontier_family:{error:?}"),
        counter_snapshot: crate::planning::FrontierCounterSnapshot::parallel_admission_denial(),
    }
}

pub(super) fn unsupported_bundle_composition_rejection() -> FrontierCertificationRejection {
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

pub(super) fn mixed_basis_bundle_rejection() -> FrontierCertificationRejection {
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

pub(super) fn hidden_serial_fallback_rejection() -> FrontierCertificationRejection {
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
