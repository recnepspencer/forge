#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct UiIntentEvidenceResourceSnapshot {
    retained_references: usize,
    retained_bytes: usize,
}

impl UiIntentEvidenceResourceSnapshot {
    pub(crate) const fn new(retained_references: usize, retained_bytes: usize) -> Self {
        Self {
            retained_references,
            retained_bytes,
        }
    }

    pub(crate) const fn retained_references(self) -> usize {
        self.retained_references
    }

    pub(crate) const fn retained_bytes(self) -> usize {
        self.retained_bytes
    }
}
