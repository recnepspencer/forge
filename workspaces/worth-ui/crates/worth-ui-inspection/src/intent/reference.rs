/// Diagnostic identity for one retained intent-evidence entry.
///
/// The scalar parts are deliberately insufficient to reconstruct any runtime
/// target, interaction, admission, attempt, or consequence authority.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct UiIntentEvidenceReference {
    session: u64,
    slot: u8,
    generation: u64,
}

impl UiIntentEvidenceReference {
    #[doc(hidden)]
    pub const fn from_diagnostic_parts(session: u64, slot: u8, generation: u64) -> Self {
        Self {
            session,
            slot,
            generation,
        }
    }

    pub const fn session_diagnostic_value(self) -> u64 {
        self.session
    }

    pub const fn slot(self) -> u8 {
        self.slot
    }

    pub const fn generation(self) -> u64 {
        self.generation
    }
}
