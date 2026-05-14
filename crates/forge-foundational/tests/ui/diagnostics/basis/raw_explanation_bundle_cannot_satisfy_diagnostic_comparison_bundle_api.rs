use forge_foundational::FoundationalDiagnosticComparisonBundle;
use forge_foundational::{
    materialize_diagnostic_explanation_bundle, DiagnosticRichnessProfile,
    FoundationalDiagnosticDeliveryClass, FoundationalDiagnosticPartiality, SupportPostureProfile,
};

#[path = "../../../certification/diagnostics/materialization_support.rs"]
mod materialization_support;

fn require_comparison_bundle(_bundle: &FoundationalDiagnosticComparisonBundle) {}

fn main() {
    let bundle = materialize_diagnostic_explanation_bundle(
        materialization_support::explanation_input(FoundationalDiagnosticPartiality::Complete),
        materialization_support::profile(
            DiagnosticRichnessProfile::Standard,
            SupportPostureProfile::SupportReady,
        ),
        FoundationalDiagnosticDeliveryClass::CanDefer,
    )
    .expect("explanation bundle");

    require_comparison_bundle(&bundle);
}
