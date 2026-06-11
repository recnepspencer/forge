use super::{
    AdmittedCapability, CapabilitySupportId, CapabilitySupportKind, CapabilitySupportPosture,
    CapabilitySupportRejection,
};

/// Required support posture for a lowering or validation boundary.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SupportRequirement {
    required: CapabilitySupportKind,
}

impl SupportRequirement {
    pub fn admitted() -> Self {
        Self {
            required: CapabilitySupportKind::Admitted,
        }
    }

    pub fn required(&self) -> CapabilitySupportKind {
        self.required
    }

    pub fn check<T: CapabilitySupportId>(
        self,
        posture: CapabilitySupportPosture<T>,
    ) -> Result<AdmittedCapability<T>, CapabilitySupportRejection<T>> {
        let (id, actual) = posture.into_id_and_kind();
        if actual == self.required {
            Ok(AdmittedCapability::from_checked_id(id))
        } else {
            Err(CapabilitySupportRejection::new(id, self.required, actual))
        }
    }
}
