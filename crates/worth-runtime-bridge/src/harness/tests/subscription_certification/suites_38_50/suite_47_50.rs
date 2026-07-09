use super::super::support::*;
use crate::facade::{
    BridgeSubscriptionTemporalAsyncCertificationCloseoutSuiteId,
    BridgeSubscriptionTemporalAsyncCertificationSupportMatrixVerdict,
};
use crate::policy::BridgeRuntimePolicy;

#[test]
fn bridge_harness_subscription_suite_47_to_50_close_the_milestone() {
    let artifact = sealed_phase_18_closeout(BridgeRuntimePolicy::development());
    let rows = artifact.support_matrix().rows();
    assert!(rows.iter().any(|row| row.suite_id()
        == BridgeSubscriptionTemporalAsyncCertificationCloseoutSuiteId::Suite47DeniedContinuation));
    assert!(rows.iter().any(|row| {
        row.suite_id()
            == BridgeSubscriptionTemporalAsyncCertificationCloseoutSuiteId::Suite48TemporalAsyncBundleParity
            && row.verdict()
                == BridgeSubscriptionTemporalAsyncCertificationSupportMatrixVerdict::ParityBandProven
    }));
    assert!(rows.iter().any(|row| row.suite_id() == BridgeSubscriptionTemporalAsyncCertificationCloseoutSuiteId::Suite49ReferenceWorkloadSufficiency));
    assert!(rows.iter().any(|row| row.suite_id()
        == BridgeSubscriptionTemporalAsyncCertificationCloseoutSuiteId::Suite50MergedCloseout));
    assert_eq!(artifact.support_matrix().rows().len(), 13);
}
