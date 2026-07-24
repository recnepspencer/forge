use std::rc::Rc;

use worth_ui_host_contract::{
    UiMountedPresentationAttemptIdentity, UiPresentationDeadline, WorthUiHostCapabilityReport,
};

use super::super::{
    UiMountedPresentationAdmission, UiMountedPresentationAdmissionDenial,
    UiMountedPresentationAdmissionRejection, UiMountedSurfaceReconciliationBinding,
};
use super::UiMountedPresentationCoordinator;

impl UiMountedPresentationCoordinator {
    pub(crate) fn admit_current(
        &mut self,
        identity: &super::super::super::UiMountedIdentityState,
        frame: super::super::super::UiPreparedMountedFrame,
        capability_report: &WorthUiHostCapabilityReport,
        deadline: UiPresentationDeadline,
        now: u64,
    ) -> Result<UiMountedPresentationAdmission, UiMountedPresentationAdmissionRejection> {
        let frame = identity.admit_prepared_frame_authority(frame)?;
        self.admit(frame, capability_report, deadline, now)
    }

    fn admit(
        &mut self,
        frame: super::super::super::UiAuthorityAdmittedMountedFrame,
        capability_report: &WorthUiHostCapabilityReport,
        deadline: UiPresentationDeadline,
        now: u64,
    ) -> Result<UiMountedPresentationAdmission, UiMountedPresentationAdmissionRejection> {
        self.admit_for(frame, capability_report, deadline, now, None)
    }

    pub(crate) fn admit_reconciliation(
        &mut self,
        frame: super::super::super::UiAuthorityAdmittedMountedFrame,
        replacements: &[UiMountedSurfaceReconciliationBinding],
        capability_report: &WorthUiHostCapabilityReport,
        deadline: UiPresentationDeadline,
        now: u64,
    ) -> Result<UiMountedPresentationAdmission, UiMountedPresentationAdmissionRejection> {
        self.admit_for(frame, capability_report, deadline, now, Some(replacements))
    }

    fn admit_for(
        &mut self,
        frame: super::super::super::UiAuthorityAdmittedMountedFrame,
        capability_report: &WorthUiHostCapabilityReport,
        deadline: UiPresentationDeadline,
        now: u64,
        reconciliation: Option<&[UiMountedSurfaceReconciliationBinding]>,
    ) -> Result<UiMountedPresentationAdmission, UiMountedPresentationAdmissionRejection> {
        let frame = frame.into_frame();
        if self.shutting_down {
            return Err(rejected(
                frame,
                UiMountedPresentationAdmissionDenial::CoordinatorShuttingDown,
            ));
        }
        if deadline.expired_at(now) {
            return Err(rejected(
                frame,
                UiMountedPresentationAdmissionDenial::DeadlineExpired,
            ));
        }
        if self.active.borrow().len() >= self.in_flight_limit {
            return Err(rejected(
                frame,
                UiMountedPresentationAdmissionDenial::CapacityExceeded,
            ));
        }
        if !self.admits_binding_purpose(&frame, reconciliation) {
            return Err(rejected(
                frame,
                UiMountedPresentationAdmissionDenial::ReconciliationBasisMismatch,
            ));
        }
        self.validate_surface_requirements(frame, capability_report, reconciliation, deadline)
    }

    fn validate_surface_requirements(
        &mut self,
        frame: super::super::super::UiPreparedMountedFrame,
        capability_report: &WorthUiHostCapabilityReport,
        reconciliation: Option<&[UiMountedSurfaceReconciliationBinding]>,
        deadline: UiPresentationDeadline,
    ) -> Result<UiMountedPresentationAdmission, UiMountedPresentationAdmissionRejection> {
        for surface in frame.surfaces() {
            let requirement = surface.requirement();
            let denial = if reconciliation.is_none()
                && self
                    .host_truth
                    .surface_requires_reconciliation(requirement.semantic_surface())
            {
                Some(
                    UiMountedPresentationAdmissionDenial::BindingRequiresReconciliation(
                        requirement.binding(),
                    ),
                )
            } else if capability_report.observation_generation()
                != requirement.capability_generation()
            {
                Some(
                    UiMountedPresentationAdmissionDenial::CapabilityGenerationChanged(
                        requirement.binding(),
                    ),
                )
            } else if capability_report.profile_identity_digest()
                != requirement.capability_profile_digest()
            {
                Some(
                    UiMountedPresentationAdmissionDenial::CapabilityProfileChanged(
                        requirement.binding(),
                    ),
                )
            } else {
                None
            };
            if let Some(denial) = denial {
                return Err(rejected(frame, denial));
            }
        }
        self.reserve_attempt(frame, deadline)
    }

    fn reserve_attempt(
        &mut self,
        frame: super::super::super::UiPreparedMountedFrame,
        deadline: UiPresentationDeadline,
    ) -> Result<UiMountedPresentationAdmission, UiMountedPresentationAdmissionRejection> {
        let attempt = match UiMountedPresentationAttemptIdentity::mint_unbound() {
            Ok(attempt) => attempt,
            Err(_) => {
                return Err(rejected(
                    frame,
                    UiMountedPresentationAdmissionDenial::IdentityExhausted,
                ));
            }
        };
        self.active.borrow_mut().insert(attempt);
        Ok(UiMountedPresentationAdmission::new(
            frame,
            attempt,
            deadline,
            Rc::clone(&self.active),
        ))
    }

    fn admits_binding_purpose(
        &self,
        frame: &super::super::super::UiPreparedMountedFrame,
        reconciliation: Option<&[UiMountedSurfaceReconciliationBinding]>,
    ) -> bool {
        let Some(replacements) = reconciliation else {
            return true;
        };
        self.host_truth.reconciliation_covers(frame, replacements)
    }
}

fn rejected(
    frame: super::super::super::UiPreparedMountedFrame,
    denial: UiMountedPresentationAdmissionDenial,
) -> UiMountedPresentationAdmissionRejection {
    UiMountedPresentationAdmissionRejection::new(frame, denial)
}
