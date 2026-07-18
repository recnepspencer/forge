#[derive(Debug, PartialEq)]
pub enum UiCommittedAllocationActivationDenialReason {
    Validation(crate::runtime::WorthUiActivationGateDenial),
    GraphPredecessorMismatch,
    LedgerPredecessorMismatch,
    ScrollBinding(crate::runtime::UiScrollOwnerCatalogDenialReport),
    PortalBinding(crate::runtime::UiPortalActivationBindingDenial),
    FrameBoundary(crate::runtime::WorthUiActivationGateDenial),
    CandidatePlanDigestMismatch,
    LedgerCommittedOutcomeMismatch,
    CommitResourceUnavailable,
    FrameReplacement(crate::runtime::UiAllocationFrameDispatchDenial),
    CounterExhausted(super::UiCommittedAllocationActivationCounterExhaustion),
}

#[derive(Debug, PartialEq)]
pub struct UiCommittedAllocationActivationDenial {
    identity: Box<super::UiCommittedAllocationActivationIdentity>,
    reason: Box<UiCommittedAllocationActivationDenialReason>,
    evidence: Box<super::UiCommittedAllocationActivationDenialEvidence>,
}

impl UiCommittedAllocationActivationDenial {
    pub fn inspection(&self) -> super::UiCommittedAllocationActivationInspection {
        super::UiCommittedAllocationActivationInspection::denied(self)
    }
    pub(super) fn validation(
        identity: super::UiCommittedAllocationActivationIdentity,
        denial: crate::runtime::WorthUiActivationGateDenial,
    ) -> Self {
        let mut counters = super::UiCommittedAllocationActivationCounters::default();
        let gate = denial.counters();
        let readiness_work = gate
            .readiness_check_count()
            .checked_add(gate.digest_check_count())
            .and_then(|count| count.checked_add(gate.query_rebind_entry_check_count()))
            .and_then(|count| count.checked_add(gate.lane_parity_check_count()));
        let counted = readiness_work
            .ok_or(super::UiCommittedAllocationActivationCounterExhaustion::ReadinessChecks)
            .and_then(|work| counters.record_readiness_checks(work))
            .and_then(|()| counters.record_denial());
        let reason = match counted {
            Ok(()) => UiCommittedAllocationActivationDenialReason::Validation(denial),
            Err(exhaustion) => {
                UiCommittedAllocationActivationDenialReason::CounterExhausted(exhaustion)
            }
        };
        let evidence =
            super::UiCommittedAllocationActivationDenialEvidence::unchanged(&identity, counters);
        Self {
            identity: Box::new(identity),
            reason: Box::new(reason),
            evidence: Box::new(evidence),
        }
    }

    pub(in crate::runtime) fn counter_exhausted(
        identity: super::UiCommittedAllocationActivationIdentity,
        counters: super::UiCommittedAllocationActivationCounters,
        exhaustion: super::UiCommittedAllocationActivationCounterExhaustion,
    ) -> Self {
        let evidence =
            super::UiCommittedAllocationActivationDenialEvidence::unchanged(&identity, counters);
        Self {
            identity: Box::new(identity),
            reason: Box::new(
                UiCommittedAllocationActivationDenialReason::CounterExhausted(exhaustion),
            ),
            evidence: Box::new(evidence),
        }
    }

    pub(in crate::runtime) fn preparation(
        identity: super::UiCommittedAllocationActivationIdentity,
        mut counters: super::UiCommittedAllocationActivationCounters,
        reason: UiCommittedAllocationActivationDenialReason,
    ) -> Self {
        let reason = match counters.record_denial() {
            Ok(()) => reason,
            Err(exhaustion) => {
                UiCommittedAllocationActivationDenialReason::CounterExhausted(exhaustion)
            }
        };
        let evidence =
            super::UiCommittedAllocationActivationDenialEvidence::unchanged(&identity, counters);
        Self {
            identity: Box::new(identity),
            reason: Box::new(reason),
            evidence: Box::new(evidence),
        }
    }

    pub fn reason(&self) -> &UiCommittedAllocationActivationDenialReason {
        &self.reason
    }
    pub fn evidence(&self) -> &super::UiCommittedAllocationActivationDenialEvidence {
        &self.evidence
    }
    pub fn attempt_identity_digest(&self) -> u64 {
        self.identity.structural_digest()
    }
}
