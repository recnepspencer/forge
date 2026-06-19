use crate::runtime::ForgeQueryGraphObligationKind;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ForgeQueryGraphObligationExecutorFamily {
    SelectionBackedDispatch,
    AdvisoryObligation,
    CapabilityGapScreen,
    PreflightSequencing,
    OperatingContextGate,
}

impl ForgeQueryGraphObligationExecutorFamily {
    pub fn from_obligation_kind(kind: ForgeQueryGraphObligationKind) -> Self {
        match kind {
            ForgeQueryGraphObligationKind::BlockingInvariant
            | ForgeQueryGraphObligationKind::SchemaContractValidator => {
                Self::SelectionBackedDispatch
            }
            ForgeQueryGraphObligationKind::AdvisoryObligation => Self::AdvisoryObligation,
            ForgeQueryGraphObligationKind::PreflightSequencingObligation => {
                Self::PreflightSequencing
            }
            ForgeQueryGraphObligationKind::CapabilityGapScreen => Self::CapabilityGapScreen,
            ForgeQueryGraphObligationKind::OperatingContextGate => Self::OperatingContextGate,
        }
    }
}
