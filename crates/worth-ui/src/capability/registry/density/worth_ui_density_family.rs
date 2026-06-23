#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum WorthUiDensityFamily {
    RowPadding,
    ContainerPadding,
    ControlSpacing,
    HitTargetMinimum,
    Posture,
}

impl WorthUiDensityFamily {
    pub(crate) fn digest_basis(&self) -> &'static str {
        match self {
            Self::RowPadding => "row_padding",
            Self::ContainerPadding => "container_padding",
            Self::ControlSpacing => "control_spacing",
            Self::HitTargetMinimum => "hit_target_minimum",
            Self::Posture => "posture",
        }
    }
}
