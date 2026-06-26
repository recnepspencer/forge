use crate::runtime::WorthUiReplacementScope;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiUnsupportedReplacementImpact {
    MissingDurableStateReceipts { scope: WorthUiReplacementScope },
    UnsupportedLane { reason: &'static str },
    UnsupportedQueryPosture { reason: &'static str },
    UnsupportedSourceCause { reason: &'static str },
    UnsupportedCapabilitySnapshot { reason: &'static str },
    UnsupportedRendererResource { reason: &'static str },
    UnsupportedPlanNodeFamily { reason: &'static str },
}
