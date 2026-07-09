#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Milestone2DigestReadinessNote {
    owns: &'static str,
    deferred: &'static str,
}

impl Milestone2DigestReadinessNote {
    pub const fn new() -> Self {
        Self {
            owns: "canonical semantic ordering and equality basis",
            deferred: "final digest algorithms, encodings, receipts, and cryptographic policy",
        }
    }

    pub const fn owns(&self) -> &'static str {
        self.owns
    }

    pub const fn deferred(&self) -> &'static str {
        self.deferred
    }
}

impl Default for Milestone2DigestReadinessNote {
    fn default() -> Self {
        Self::new()
    }
}
