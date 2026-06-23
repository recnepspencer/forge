use crate::runtime::WorthUiRuntimeFactFamily;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiSemanticSliceFactMapping {
    Exact(WorthUiRuntimeFactFamily),
    Composite(&'static [WorthUiRuntimeFactFamily]),
    Gap,
}

impl WorthUiSemanticSliceFactMapping {
    pub fn exact_family(self) -> Option<WorthUiRuntimeFactFamily> {
        match self {
            Self::Exact(family) => Some(family),
            Self::Composite(_) | Self::Gap => None,
        }
    }

    pub fn contains_family(self, family: WorthUiRuntimeFactFamily) -> bool {
        match self {
            Self::Exact(exact) => exact == family,
            Self::Composite(families) => families.contains(&family),
            Self::Gap => false,
        }
    }
}
