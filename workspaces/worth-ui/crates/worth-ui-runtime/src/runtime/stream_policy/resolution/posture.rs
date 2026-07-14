use super::*;

pub(super) fn resolve_ingress_policy_verdict(
    fact: &UiAllocationFrameSourceFact,
    family: UiAllocationStreamFamily,
) -> UiAllocationIngressPolicyVerdict {
    match fact {
        UiAllocationFrameSourceFact::QueryProjection {
            posture: UiAllocationFrameQuerySettlementPosture::Partial,
            warnings,
            ..
        } => {
            let policy =
                crate::runtime::stream_policy::UiAllocationStreamPolicy::for_family(family);
            debug_assert_eq!(
                policy.partial_settlement_law(),
                UiAllocationPartialSettlementLaw::StaleButBounded
            );
            UiAllocationIngressPolicyVerdict::PartialQueryStaleButBounded {
                warnings: *warnings,
                max_lag_frames: policy.budget().max_lag_frames(),
            }
        }
        _ => UiAllocationIngressPolicyVerdict::Current,
    }
}
