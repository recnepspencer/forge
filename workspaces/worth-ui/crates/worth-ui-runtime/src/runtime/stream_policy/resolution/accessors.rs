use super::*;

impl UiResolvedAllocationFramePlan {
    pub(crate) fn into_narrowing_parts(
        self,
    ) -> (
        UiAllocationFramePlanIdentity,
        Box<[UiAllocationFrameSourceFact]>,
    ) {
        (self.identity, self.sources)
    }
    pub(super) fn receipt(&self) -> &UiAllocationStreamPolicyEvidenceReceipt {
        self.identity.receipt()
    }
    pub fn frame_epoch(&self) -> crate::runtime::UiAllocationFrameEpoch {
        self.receipt().epoch()
    }
    pub fn families(&self) -> &[UiAllocationStreamFamily] {
        self.receipt().families()
    }
    pub fn invalidations(&self) -> &[UiAllocationInvalidationIntent] {
        self.receipt().invalidations()
    }
    pub fn policy(&self) -> UiResolvedAllocationStreamPolicy {
        self.receipt().policy()
    }
    pub fn identity(&self) -> &UiAllocationFramePlanIdentity {
        &self.identity
    }
    pub fn intermediate_policy_verdicts(
        &self,
    ) -> &[crate::runtime::UiAllocationIntermediatePolicyVerdict] {
        self.receipt().intermediate()
    }
    pub fn policy_branches(&self) -> &[crate::runtime::UiResolvedAllocationPolicyBranch] {
        self.receipt().branches()
    }
    pub fn order_verdicts(&self) -> &[UiAllocationSourceOrderVerdict] {
        self.receipt().order_verdicts()
    }
    pub fn duplicate_posture(&self) -> UiAllocationDuplicatePosture<'_> {
        UiAllocationDuplicatePosture {
            witness: self.receipt().duplicate_witness(),
        }
    }
    pub fn cadence_verdict(&self) -> UiAllocationFrameCadenceVerdict {
        self.receipt().cadence()
    }
    pub fn invalidation_ingress_key(
        &self,
        invalidation: &UiAllocationInvalidationIntent,
    ) -> Result<&UiAllocationFrameIngressKey, UiAllocationInvalidationReferenceDenial> {
        let ingress_ref = invalidation.ingress_ref();
        let plan_epoch = self.frame_epoch();
        if ingress_ref.epoch() != plan_epoch {
            return Err(
                UiAllocationInvalidationReferenceDenial::FrameEpochMismatch {
                    plan: plan_epoch,
                    invalidation: ingress_ref.epoch(),
                },
            );
        }
        let ordinal = usize::from(ingress_ref.ordinal());
        let owned_invalidation = self.invalidations().get(ordinal).ok_or(
            UiAllocationInvalidationReferenceDenial::MissingCanonicalIngress {
                ordinal: ingress_ref.ordinal(),
                ingress_count: self.receipt().ingress_keys().len() as u16,
            },
        )?;
        if !std::ptr::eq(owned_invalidation, invalidation) {
            return Err(
                UiAllocationInvalidationReferenceDenial::ForeignPlanInvalidation {
                    plan: plan_epoch,
                    invalidation: ingress_ref.epoch(),
                    ordinal: ingress_ref.ordinal(),
                },
            );
        }
        Ok(&self.receipt().ingress_keys()[ordinal])
    }
    pub fn counters(&self) -> UiAllocationFrameResolutionCounters {
        self.counters
    }
    pub fn evidence(&self) -> &UiAllocationStreamPolicyEvidenceOutcome {
        &self.identity.evidence
    }
    pub fn ingress_policy_verdicts(&self) -> &[UiAllocationIngressPolicyVerdict] {
        self.receipt().ingress_policy_verdicts()
    }
}

impl UiAllocationFramePlanIdentity {
    pub(super) fn receipt(&self) -> &UiAllocationStreamPolicyEvidenceReceipt {
        self.evidence
            .resolved()
            .expect("accepted plan identity owns resolved evidence")
    }
    pub fn epoch(&self) -> crate::runtime::UiAllocationFrameEpoch {
        self.receipt().epoch()
    }
    pub fn ingress_keys(&self) -> &[UiAllocationFrameIngressKey] {
        self.receipt().ingress_keys()
    }
    pub fn families(&self) -> &[UiAllocationStreamFamily] {
        self.receipt().families()
    }
    pub fn order_verdicts(&self) -> &[UiAllocationSourceOrderVerdict] {
        self.receipt().order_verdicts()
    }
    pub fn duplicate_posture(&self) -> UiAllocationDuplicatePosture<'_> {
        UiAllocationDuplicatePosture {
            witness: self.receipt().duplicate_witness(),
        }
    }
    pub fn invalidations(&self) -> &[UiAllocationInvalidationIntent] {
        self.receipt().invalidations()
    }
    pub fn policy(&self) -> UiResolvedAllocationStreamPolicy {
        self.receipt().policy()
    }
    pub fn intermediate_policy_verdicts(
        &self,
    ) -> &[crate::runtime::UiAllocationIntermediatePolicyVerdict] {
        self.receipt().intermediate()
    }
    pub fn policy_branches(&self) -> &[crate::runtime::UiResolvedAllocationPolicyBranch] {
        self.receipt().branches()
    }
    pub fn cadence(&self) -> UiAllocationFrameCadenceVerdict {
        self.receipt().cadence()
    }
    pub fn evidence(&self) -> &UiAllocationStreamPolicyEvidenceOutcome {
        &self.evidence
    }
    pub fn ingress_policy_verdicts(&self) -> &[UiAllocationIngressPolicyVerdict] {
        self.receipt().ingress_policy_verdicts()
    }
}

impl UiAllocationFrameRejection {
    fn receipt(&self) -> &UiAllocationStreamPolicyDenialEvidenceReceipt {
        self.evidence
            .denied()
            .expect("rejection owns denial evidence")
    }
    pub fn epoch(&self) -> crate::runtime::UiAllocationFrameEpoch {
        self.receipt().epoch()
    }
    pub fn ingress_keys(&self) -> &[UiAllocationFrameIngressKey] {
        self.receipt().ingress_keys()
    }
    pub fn denial(&self) -> UiAllocationFrameResolutionDenial {
        self.receipt().denial()
    }
    pub fn evidence(&self) -> &UiAllocationStreamPolicyEvidenceOutcome {
        &self.evidence
    }
}

impl UiAllocationDuplicatePosture<'_> {
    pub fn witness(&self) -> &UiAllocationFrameDuplicateWitness {
        self.witness
    }
}

#[rustfmt::skip]
impl UiAllocationFrameResolutionCounters {
    pub fn entry_visits(self) -> u16 { self.entry_visits }
    pub fn gap_checks(self) -> u16 { self.gap_checks }
    pub fn policy_family_count(self) -> u8 { self.policy_family_count }
    pub fn invalidation_count(self) -> u16 { self.invalidation_count }
    pub fn order_ledger_scans(self) -> u16 { self.order_ledger_scans }
    pub fn order_ledger_writes(self) -> u16 { self.order_ledger_writes }
}
