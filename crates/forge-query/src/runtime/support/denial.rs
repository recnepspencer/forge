use super::{
    ForgeQueryRuntimeFacadeFamily, ForgeQueryRuntimeFamilySupportStatus,
    ForgeQueryRuntimeFamilyTeachingPosture,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryRuntimeSupportDenial {
    family: ForgeQueryRuntimeFacadeFamily,
    status: ForgeQueryRuntimeFamilySupportStatus,
    teaching_posture: Option<ForgeQueryRuntimeFamilyTeachingPosture>,
    reason: String,
}

impl ForgeQueryRuntimeSupportDenial {
    pub(crate) fn new(
        family: ForgeQueryRuntimeFacadeFamily,
        status: ForgeQueryRuntimeFamilySupportStatus,
        teaching_posture: Option<ForgeQueryRuntimeFamilyTeachingPosture>,
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
        family: ForgeQueryRuntimeFacadeFamily,
        reason: impl Into<String>,
    ) -> Self {
        Self::new(
            family,
            ForgeQueryRuntimeFamilySupportStatus::Unsupported,
            None,
            reason,
        )
    }

    pub fn family(&self) -> ForgeQueryRuntimeFacadeFamily {
        self.family
    }

    pub fn status(&self) -> ForgeQueryRuntimeFamilySupportStatus {
        self.status
    }

    pub fn teaching_posture(&self) -> Option<ForgeQueryRuntimeFamilyTeachingPosture> {
        self.teaching_posture
    }

    pub fn reason(&self) -> &str {
        &self.reason
    }
}

impl std::fmt::Display for ForgeQueryRuntimeSupportDenial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "runtime backend does not admit `{}` facade family: {}",
            self.family, self.reason
        )
    }
}
