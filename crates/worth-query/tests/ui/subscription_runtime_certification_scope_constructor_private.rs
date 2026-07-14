use worth_query::facade::runtime::{WorthQueryEvidenceIdentity, WorthQueryEvidenceScope, QuerySubscriptionAdmittedDiagnosticBundle, QuerySubscriptionBridgeParityExplanation, QuerySubscriptionSupportReport};
use worth_query::facade::certification::{CertifiedFamilyCoverageHandle, QuerySubscriptionRuntimeCertificationCounters, QuerySubscriptionRuntimeCertificationScope, SubscriptionLifecycleCertificationBundle};

fn main() {
    let support_report: QuerySubscriptionSupportReport = todo!();
    let bridge_parity: QuerySubscriptionBridgeParityExplanation = todo!();
    let admitted_diagnostic_bundle: QuerySubscriptionAdmittedDiagnosticBundle = todo!();
    let lifecycle_certification: SubscriptionLifecycleCertificationBundle = todo!();
    let coverage_handle: CertifiedFamilyCoverageHandle = todo!();

    let _ = QuerySubscriptionRuntimeCertificationScope {
        family: support_report.support_subject().family().clone(),
        support_report,
        bridge_parity,
        admitted_diagnostic_bundle,
        lifecycle_certification,
        coverage_handle,
        scope_identity: WorthQueryEvidenceIdentity::compose(
            WorthQueryEvidenceScope::SubscriptionActivationReceipt,
        )
        .seal(),
        counters: QuerySubscriptionRuntimeCertificationCounters::default(),
    };
}
