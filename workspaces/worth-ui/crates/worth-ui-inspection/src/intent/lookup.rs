use super::UiIntentCausalTraceEvidence;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiIntentEvidenceLookup {
    Found(UiIntentCausalTraceEvidence),
    Expired,
    ForeignSession,
}
