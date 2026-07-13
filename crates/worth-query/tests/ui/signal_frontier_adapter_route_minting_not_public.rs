use worth_query::facade::foundation::{FrontierDisjointnessClass, FrontierPredictionDriftOutcome, SerialFallbackReason, SignalFrontierSurfaceEvidence};
use worth_query::facade::SignalFrontierBundleEvidence;

fn assert_route_minting_is_private(
    surface: SignalFrontierSurfaceEvidence,
    bundle: SignalFrontierBundleEvidence,
) {
    let _ = surface.to_parallel_admission_evidence(
        "basis",
        FrontierDisjointnessClass::CollectionWindowSurface,
    );
    let _ = surface.to_serial_fallback_evidence(
        "basis",
        SerialFallbackReason::DeterministicAdmissionDenied,
        FrontierPredictionDriftOutcome::WithinBudget,
    );

    let _ = bundle.bind_to_basis("basis");
}

fn main() {}
