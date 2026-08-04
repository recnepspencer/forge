use super::super::identity::{
    LiveChangeOrdinal, LiveProgressBasis, LiveProgressError, LiveStartBasis, LiveSubscriptionDigest,
};
use super::super::refresh::{
    CoalescingDecision, LiveCoalescingError, LiveRefreshError, PatchWidthAssessment,
    PatchWidthResolution, RefreshAdmissionClass, RefreshFallback,
};
use super::super::relevance::{BridgeChangeSummary, ChangeRelevance};
use super::descriptor::LivePromotionDescriptor;
use crate::basis::ResolvedSnapshotBasis;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LiveQueryPlan {
    pub(in crate::live) descriptor: LivePromotionDescriptor,
    pub(in crate::live) start_basis: LiveStartBasis,
    pub(in crate::live) progress_basis: LiveProgressBasis,
    pub(in crate::live) subscription_digest: LiveSubscriptionDigest,
    pub(in crate::live) baseline_result_digest: String,
}

impl LiveQueryPlan {
    pub fn descriptor(&self) -> &LivePromotionDescriptor {
        &self.descriptor
    }

    pub fn start_basis(&self) -> &LiveStartBasis {
        &self.start_basis
    }

    pub fn progress_basis(&self) -> &LiveProgressBasis {
        &self.progress_basis
    }

    pub fn subscription_digest(&self) -> &LiveSubscriptionDigest {
        &self.subscription_digest
    }

    pub fn baseline_result_digest(&self) -> &str {
        &self.baseline_result_digest
    }

    pub fn performance_status(&self) -> &str {
        self.descriptor.performance_report().performance_status()
    }

    pub fn advance_progress(
        &self,
        next_ordinal: LiveChangeOrdinal,
        next_basis: ResolvedSnapshotBasis,
    ) -> Result<Self, LiveProgressError> {
        let progress_basis = self.progress_basis.advance(
            self.progress_basis.change_sequence_id(),
            next_ordinal,
            next_basis,
        )?;
        Ok(Self {
            descriptor: self.descriptor.clone(),
            start_basis: self.start_basis.clone(),
            progress_basis,
            subscription_digest: self.subscription_digest.clone(),
            baseline_result_digest: self.baseline_result_digest.clone(),
        })
    }

    pub fn evaluate_delivery_width(&self, measured_width: usize) -> PatchWidthAssessment {
        let budget_limit = self.descriptor.performance_report().width_budget().limit();
        if measured_width <= budget_limit {
            return PatchWidthAssessment {
                measured_width,
                budget_limit,
                resolution: PatchWidthResolution::Deliver,
            };
        }

        let resolution = match self.descriptor.performance_report().width_policy() {
            crate::live_performance::PatchWidthPolicy::DeliverWithinBudget => {
                PatchWidthResolution::Reject
            }
            crate::live_performance::PatchWidthPolicy::CoalesceWithinAdmittedClass => {
                PatchWidthResolution::Coalesce
            }
            crate::live_performance::PatchWidthPolicy::RefreshWithinAdmissionMatrix => {
                if self
                    .descriptor
                    .refresh_admission_matrix()
                    .admits(&RefreshAdmissionClass::WidthOverflow)
                {
                    PatchWidthResolution::Refresh(RefreshFallback {
                        admission_class: RefreshAdmissionClass::WidthOverflow,
                        cost_class: self
                            .descriptor
                            .performance_report()
                            .refresh_cost_class()
                            .clone(),
                        admission_status: self
                            .descriptor
                            .performance_report()
                            .refresh_admission_status()
                            .clone(),
                    })
                } else {
                    PatchWidthResolution::Reject
                }
            }
            crate::live_performance::PatchWidthPolicy::RejectOverflow => {
                PatchWidthResolution::Reject
            }
        };

        PatchWidthAssessment {
            measured_width,
            budget_limit,
            resolution,
        }
    }

    pub fn request_coalesced_delivery(
        &self,
        bundle_count: usize,
    ) -> Result<CoalescingDecision, LiveCoalescingError> {
        if bundle_count == 0 {
            return Err(LiveCoalescingError::BundleCountTooSmall);
        }
        if bundle_count == 1 {
            return Ok(CoalescingDecision::NotNeeded);
        }

        match self.descriptor.performance_report().coalescing_admission() {
            crate::live_performance::CoalescingAdmissionClass::BasisStableEquivalent => {
                Ok(CoalescingDecision::Admitted { bundle_count })
            }
            crate::live_performance::CoalescingAdmissionClass::Forbidden => {
                Err(LiveCoalescingError::Forbidden)
            }
        }
    }

    pub fn request_refresh_fallback(
        &self,
        admission_class: RefreshAdmissionClass,
    ) -> Result<RefreshFallback, LiveRefreshError> {
        if self
            .descriptor
            .refresh_admission_matrix()
            .admits(&admission_class)
            && self
                .descriptor
                .performance_report()
                .refresh_admission_status()
                != &crate::live_performance::RefreshAdmissionStatus::Forbidden
        {
            Ok(RefreshFallback {
                admission_class,
                cost_class: self
                    .descriptor
                    .performance_report()
                    .refresh_cost_class()
                    .clone(),
                admission_status: self
                    .descriptor
                    .performance_report()
                    .refresh_admission_status()
                    .clone(),
            })
        } else {
            Err(LiveRefreshError::ForbiddenAdmissionClass(admission_class))
        }
    }

    pub fn classify_change(&self, change: &BridgeChangeSummary) -> ChangeRelevance {
        self.descriptor.relevance_contract().classify_change(change)
    }
}
