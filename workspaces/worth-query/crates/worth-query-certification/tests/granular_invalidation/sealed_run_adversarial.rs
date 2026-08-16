use worth_runtime_bridge::facade::BridgeDiagnosticsTier;

use crate::production_evidence::CertificationComparatorPolicy;
use crate::production_scenarios::run_scenario;
use crate::sealed_run::{
    production_claims, verify_evidence, GranularInvalidationCertificationRun,
};
use crate::world::GranularInvalidationScenario;

pub fn assert_adversarial_sealing_denials() {
    let ordered = production_claims(17);
    let mut reversed = ordered.clone();
    reversed.reverse();
    let ordered = GranularInvalidationCertificationRun::seal(ordered)
        .expect("the canonical scenario order must seal");
    let reversed = GranularInvalidationCertificationRun::seal(reversed)
        .expect("claim input order must not affect sealing");
    assert_eq!(ordered.report_digest(), reversed.report_digest());

    let mut missing = production_claims(17);
    missing.pop();
    assert!(GranularInvalidationCertificationRun::seal(missing).is_err());

    let mut duplicate = production_claims(17);
    duplicate[0].scenario = duplicate[1].scenario;
    assert!(GranularInvalidationCertificationRun::seal(duplicate).is_err());

    let mut wrong_seed = production_claims(17);
    wrong_seed[0].seed = 18;
    assert!(GranularInvalidationCertificationRun::seal(wrong_seed).is_err());

    let scenario = GranularInvalidationScenario::CurveDetailToLiveRisk;
    let evidence = run_scenario(scenario, 17)
        .with_faulted_scenario(GranularInvalidationScenario::SuppressedQuoteNoQueryPatch);
    assert!(verify_evidence(scenario, 17, evidence).is_err());
    assert!(verify_evidence(
        scenario,
        17,
        run_scenario(scenario, 17).with_faulted_seed(18)
    )
    .is_err());
    assert!(verify_evidence(
        scenario,
        17,
        run_scenario(scenario, 17).with_faulted_policy(
            CertificationComparatorPolicy::Tolerance {
                epsilon: 999,
                provider_identity: "forged-provider",
            }
        )
    )
    .is_err());
    assert!(verify_evidence(
        scenario,
        17,
        run_scenario(scenario, 17).with_faulted_tier(BridgeDiagnosticsTier::Exhaustive)
    )
    .is_err());
    assert!(verify_evidence(
        scenario,
        17,
        run_scenario(scenario, 17).with_faulted_runtime_generation(0)
    )
    .is_err());
    assert!(verify_evidence(
        scenario,
        17,
        run_scenario(scenario, 17).with_faulted_direct_truth_count(999)
    )
    .is_err());
    assert!(verify_evidence(
        scenario,
        17,
        run_scenario(scenario, 17).with_faulted_signal_identity("forged:signal")
    )
    .is_err());
}
