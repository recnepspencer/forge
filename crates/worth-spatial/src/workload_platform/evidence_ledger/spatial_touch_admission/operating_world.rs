#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpatialGeometryEvidenceTouchOperatingWorld {
    CurrentHead,
}

impl SpatialGeometryEvidenceTouchOperatingWorld {
    pub(crate) const fn current_head() -> Self {
        Self::CurrentHead
    }

    pub fn posture(self) -> &'static str {
        match self {
            Self::CurrentHead => "current-head",
        }
    }

    pub(crate) fn digest_key(self) -> String {
        format!("operating-world:{}", self.posture())
    }
}
