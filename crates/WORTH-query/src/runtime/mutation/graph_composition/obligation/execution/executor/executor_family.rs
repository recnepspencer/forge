use crate::runtime::WorthQueryGraphObligationKind;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum WorthQueryGraphObligationExecutorFamily {
    SelectionBackedDispatch,
    AdvisoryObligation,
    CapabilityGapScreen,
    PreflightSequencing,
    OperatingContextGate,
}

impl WorthQueryGraphObligationExecutorFamily {
    pub fn from_obligation_kind(kind: WorthQueryGraphObligationKind) -> Self {
        match kind {
            WorthQueryGraphObligationKind::BlockingInvariant
            | WorthQueryGraphObligationKind::SchemaContractValidator => {
                Self::SelectionBackedDispatch
            }
            WorthQueryGraphObligationKind::AdvisoryObligation => Self::AdvisoryObligation,
            WorthQueryGraphObligationKind::PreflightSequencingObligation => {
                Self::PreflightSequencing
            }
            WorthQueryGraphObligationKind::CapabilityGapScreen => Self::CapabilityGapScreen,
            WorthQueryGraphObligationKind::OperatingContextGate => Self::OperatingContextGate,
        }
    }
}
