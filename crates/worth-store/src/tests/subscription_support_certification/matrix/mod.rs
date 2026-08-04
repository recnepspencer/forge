mod authority;
mod budgets;
mod compatibility;
mod drift;
mod evidence;
mod exact_recovery;
mod hidden_loss;
mod lifecycle;
mod maintenance;
mod pipeline;
mod portability;
mod retention;
use super::evidence::CertificationMatrixEvidence;

use super::super::{
    SubscriptionSupportCatalog, SubscriptionSupportCertificationBundle,
    SubscriptionSupportCertificationLaneKind, SubscriptionSupportCertificationMatrixStatus,
    SubscriptionSupportCounterSnapshot,
};

use self::authority::record_authority;
use self::budgets::record_budgets;
use self::compatibility::record_compatibility;
use self::drift::record_drift;
use self::exact_recovery::record_exact_recovery;
use self::hidden_loss::record_hidden_loss;
use self::lifecycle::record_lifecycle;
use self::maintenance::record_maintenance;
use self::pipeline::record_pipeline;
use self::portability::record_portability;
use self::retention::record_retention;

#[test]
fn durable_subscription_support_resume_contract_phase_6a_matrix_is_machine_checkable() {
    let mut evidence = CertificationMatrixEvidence::new();
    record_exact_recovery(&mut evidence);
    record_drift(&mut evidence);
    record_lifecycle(&mut evidence);
    record_authority(&mut evidence);
    record_budgets(&mut evidence);
    record_compatibility(&mut evidence);
    record_retention(&mut evidence);
    record_portability(&mut evidence);
    record_maintenance(&mut evidence);
    record_pipeline(&mut evidence);
    record_hidden_loss(&mut evidence);
    let (classification_reports, lane_outcomes) = evidence.into_parts();
    let bundle = SubscriptionSupportCertificationBundle::from_lane_outcomes(
        &SubscriptionSupportCatalog::first_ship(),
        SubscriptionSupportCounterSnapshot::default(),
        &classification_reports,
        lane_outcomes,
    )
    .unwrap();

    let matrix = bundle
        .matrix()
        .expect("Phase 6A bundle must carry a matrix");
    assert_eq!(
        matrix.status(),
        SubscriptionSupportCertificationMatrixStatus::Phase6AOperationalParticipationComplete
    );
    assert_eq!(
        matrix.lane_outcomes().len(),
        SubscriptionSupportCertificationLaneKind::phase_6a_required().len()
    );
    assert_eq!(bundle.catalog_family_count(), 3);
    assert!(!bundle.truth_digest().is_empty());
    assert!(!bundle.artifact_digest().is_empty());
    assert!(!bundle.subscription_support_digest().is_empty());
    assert!(!bundle.replay_digest().is_empty());
    assert!(!bundle.diagnostics_digest().is_empty());
    assert!(!bundle.failure_digest().is_empty());
    assert!(!bundle.counter_digest().is_empty());
}
