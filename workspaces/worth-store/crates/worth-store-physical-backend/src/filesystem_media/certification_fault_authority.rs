use super::{
    MediaFaultDirective, MediaFaultRule, MediaFaultSchedule, MediaFaultScheduleDenial,
    MediaOperationRole, MediaPauseGate,
};

/// Certification-only authority for deterministic faults on the real backend.
///
/// The authority is issued through the Store admission surface. Raw fault
/// rules and schedules have no public constructor, so enabling the feature
/// does not itself create a parallel backend or runtime owner.
#[derive(Debug, Clone, Copy)]
pub struct CertificationMediaFaultAuthority {
    _private: (),
}

impl CertificationMediaFaultAuthority {
    pub(super) const fn from_filesystem_admission() -> Self {
        Self { _private: () }
    }

    pub fn rule(
        &self,
        role: MediaOperationRole,
        ordinal: u64,
        directive: MediaFaultDirective,
    ) -> MediaFaultRule {
        MediaFaultRule::for_certification(role, ordinal, directive)
    }

    pub fn schedule(
        &self,
        rules: Vec<MediaFaultRule>,
    ) -> Result<MediaFaultSchedule, MediaFaultScheduleDenial> {
        MediaFaultSchedule::for_certification(rules)
    }

    pub fn pause_gate(&self) -> MediaPauseGate {
        MediaPauseGate::for_certification()
    }
}

#[doc(hidden)]
pub const fn certification_media_fault_authority() -> CertificationMediaFaultAuthority {
    CertificationMediaFaultAuthority::from_filesystem_admission()
}
