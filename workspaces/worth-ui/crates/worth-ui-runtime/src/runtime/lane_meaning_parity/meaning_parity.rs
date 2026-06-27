use crate::runtime::WorthUiCrossLaneSemanticReference;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiLaneMeaningParity {
    reference: WorthUiCrossLaneSemanticReference,
}

impl WorthUiLaneMeaningParity {
    pub(crate) fn certified(reference: WorthUiCrossLaneSemanticReference) -> Self {
        Self { reference }
    }

    pub fn reference(&self) -> &WorthUiCrossLaneSemanticReference {
        &self.reference
    }
}
