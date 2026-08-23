use std::collections::BTreeSet;

const PHYSICAL_RECOVERY_CAPACITY: usize = 64;

#[derive(Default)]
pub(super) struct UiNativePhysicalRecoveryTracker {
    expected: BTreeSet<(
        worth_ui_host_contract::UiMountedPresentationAttemptIdentity,
        worth_ui_host_contract::UiSurfaceBindingGeneration,
    )>,
    pending: BTreeSet<worth_ui_host_native::UiNativePhysicalPresentationCorrelation>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum UiNativePhysicalRecoveryTrackingDenial {
    CapacityExceeded,
    DuplicateCorrelation,
    UnknownCorrelation,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum UiNativePhysicalRecoverySettlement {
    AttemptStillPending,
    AttemptReady,
}

impl UiNativePhysicalRecoveryTracker {
    pub(super) fn expect(
        &mut self,
        attempt: worth_ui_host_contract::UiMountedPresentationAttemptIdentity,
        binding: worth_ui_host_contract::UiSurfaceBindingGeneration,
    ) -> Result<(), UiNativePhysicalRecoveryTrackingDenial> {
        if self.expected.len() + self.pending.len() >= PHYSICAL_RECOVERY_CAPACITY {
            return Err(UiNativePhysicalRecoveryTrackingDenial::CapacityExceeded);
        }
        self.expected
            .insert((attempt, binding))
            .then_some(())
            .ok_or(UiNativePhysicalRecoveryTrackingDenial::DuplicateCorrelation)
    }

    pub(super) fn observe_scheduled(
        &mut self,
        correlation: worth_ui_host_native::UiNativePhysicalPresentationCorrelation,
    ) -> Result<(), UiNativePhysicalRecoveryTrackingDenial> {
        if self.pending.contains(&correlation) {
            return Err(UiNativePhysicalRecoveryTrackingDenial::DuplicateCorrelation);
        }
        self.expected
            .remove(&(correlation.attempt(), correlation.binding()))
            .then_some(())
            .ok_or(UiNativePhysicalRecoveryTrackingDenial::UnknownCorrelation)?;
        self.pending.insert(correlation);
        Ok(())
    }

    pub(super) fn classify_settlement(
        &self,
        correlation: worth_ui_host_native::UiNativePhysicalPresentationCorrelation,
    ) -> Result<UiNativePhysicalRecoverySettlement, UiNativePhysicalRecoveryTrackingDenial> {
        if !self.pending.contains(&correlation) {
            return Err(UiNativePhysicalRecoveryTrackingDenial::UnknownCorrelation);
        }
        Ok(
            if self.pending.iter().any(|pending| {
                *pending != correlation && pending.attempt() == correlation.attempt()
            }) {
                UiNativePhysicalRecoverySettlement::AttemptStillPending
            } else {
                UiNativePhysicalRecoverySettlement::AttemptReady
            },
        )
    }

    pub(super) fn commit_settlement(
        &mut self,
        correlation: worth_ui_host_native::UiNativePhysicalPresentationCorrelation,
    ) -> Result<(), UiNativePhysicalRecoveryTrackingDenial> {
        self.pending
            .remove(&correlation)
            .then_some(())
            .ok_or(UiNativePhysicalRecoveryTrackingDenial::UnknownCorrelation)
    }

    pub(super) fn is_empty(&self) -> bool {
        self.expected.is_empty() && self.pending.is_empty()
    }

    pub(super) fn has_pending(&self) -> bool {
        !self.is_empty()
    }
}

#[cfg(all(test, feature = "certification-support"))]
mod tests {
    use super::*;

    #[test]
    fn two_surface_recovery_waits_for_both_exact_physical_correlations() {
        let attempt =
            worth_ui_host_contract::UiMountedPresentationAttemptIdentity::mint_unbound().unwrap();
        let left = correlation(attempt, 1);
        let right = correlation(attempt, 2);
        let foreign = correlation(
            worth_ui_host_contract::UiMountedPresentationAttemptIdentity::mint_unbound().unwrap(),
            3,
        );
        let mut tracker = UiNativePhysicalRecoveryTracker::default();
        tracker.expect(left.attempt(), left.binding()).unwrap();
        tracker.expect(right.attempt(), right.binding()).unwrap();
        tracker.observe_scheduled(left).unwrap();
        tracker.observe_scheduled(right).unwrap();
        assert_eq!(
            tracker.observe_scheduled(left),
            Err(UiNativePhysicalRecoveryTrackingDenial::DuplicateCorrelation)
        );
        assert_eq!(
            tracker.classify_settlement(left),
            Ok(UiNativePhysicalRecoverySettlement::AttemptStillPending)
        );
        tracker.commit_settlement(left).unwrap();
        assert_eq!(
            tracker.classify_settlement(foreign),
            Err(UiNativePhysicalRecoveryTrackingDenial::UnknownCorrelation)
        );
        assert_eq!(
            tracker.classify_settlement(right),
            Ok(UiNativePhysicalRecoverySettlement::AttemptReady)
        );
        assert_eq!(
            tracker.classify_settlement(right),
            Ok(UiNativePhysicalRecoverySettlement::AttemptReady),
            "fallible reconstruction may retry before consuming exact recovery authority"
        );
        tracker.commit_settlement(right).unwrap();
        assert!(tracker.is_empty());
    }

    fn correlation(
        attempt: worth_ui_host_contract::UiMountedPresentationAttemptIdentity,
        sequence: u64,
    ) -> worth_ui_host_native::UiNativePhysicalPresentationCorrelation {
        worth_ui_host_native::UiNativePhysicalPresentationCorrelation::from_certification(
            attempt,
            worth_ui_host_contract::UiSemanticSurfaceIdentity::mint_unbound().unwrap(),
            worth_ui_host_contract::UiSurfaceBindingGeneration::mint_unbound().unwrap(),
            sequence,
        )
        .unwrap()
    }
}
