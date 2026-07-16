use super::RuntimeOutcomeFamily;

/// UI-owned outcome presentation source. Upstream runtimes translate their
/// postures at their binding edge; Worth UI retains only the presentation
/// family it actually owns.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RuntimeOutcomeSourceReference {
    family: RuntimeOutcomeFamily,
}

impl RuntimeOutcomeSourceReference {
    pub fn new(family: RuntimeOutcomeFamily) -> Self {
        Self { family }
    }

    pub(crate) fn admits_family(&self, family: &RuntimeOutcomeFamily) -> bool {
        &self.family == family
    }

    pub(crate) fn digest_basis(&self) -> String {
        format!("ui_outcome_family:{}", self.family.digest_basis())
    }
}
