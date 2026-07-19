use super::{
    WorthQueryRuntimeFacadeFamily, WorthQueryRuntimeFamilySupportStatus,
    WorthQueryRuntimeFamilyTeachingPosture,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryRuntimeSupportDenial {
    family: WorthQueryRuntimeFacadeFamily,
    status: WorthQueryRuntimeFamilySupportStatus,
    teaching_posture: Option<WorthQueryRuntimeFamilyTeachingPosture>,
    reason: String,
}

impl WorthQueryRuntimeSupportDenial {
    pub(crate) fn new(
        family: WorthQueryRuntimeFacadeFamily,
        status: WorthQueryRuntimeFamilySupportStatus,
        teaching_posture: Option<WorthQueryRuntimeFamilyTeachingPosture>,
        reason: impl Into<String>,
    ) -> Self {
        Self {
            family,
            status,
            teaching_posture,
            reason: reason.into(),
        }
    }

    pub(crate) fn unsupported(
        family: WorthQueryRuntimeFacadeFamily,
        reason: impl Into<String>,
    ) -> Self {
        Self::new(
            family,
            WorthQueryRuntimeFamilySupportStatus::Unsupported,
            None,
            reason,
        )
    }

    pub fn family(&self) -> WorthQueryRuntimeFacadeFamily {
        self.family
    }

    pub fn status(&self) -> WorthQueryRuntimeFamilySupportStatus {
        self.status
    }

    pub fn teaching_posture(&self) -> Option<WorthQueryRuntimeFamilyTeachingPosture> {
        self.teaching_posture
    }

    pub fn reason(&self) -> &str {
        &self.reason
    }
}

impl std::fmt::Display for WorthQueryRuntimeSupportDenial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "runtime backend does not admit `{}` facade family: {}",
            self.family, self.reason
        )
    }
}
