use forge_query::facade::{
    ForgeQueryGraphReadAccessAdmissionPosture, ForgeQueryGraphReadAccessDenialKind,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadExpectedDenial {
    denial_kind: ForgeQueryGraphReadAccessDenialKind,
    suggested_posture: ForgeQueryGraphReadAccessAdmissionPosture,
}

impl WorthGraphReadExpectedDenial {
    pub const fn new(
        denial_kind: ForgeQueryGraphReadAccessDenialKind,
        suggested_posture: ForgeQueryGraphReadAccessAdmissionPosture,
    ) -> Self {
        Self {
            denial_kind,
            suggested_posture,
        }
    }

    pub fn denial_kind(&self) -> &ForgeQueryGraphReadAccessDenialKind {
        &self.denial_kind
    }

    pub fn suggested_posture(&self) -> &ForgeQueryGraphReadAccessAdmissionPosture {
        &self.suggested_posture
    }
}
