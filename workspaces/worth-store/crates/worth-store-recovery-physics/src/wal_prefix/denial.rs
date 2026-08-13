#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum WalPrefixAdmissionDenial {
    FrontierMismatch,
    Gap,
    InterruptedMiddle,
}
