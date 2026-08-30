#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum UiCommandShortcutSequence {
    Single(super::UiCommandShortcutStroke),
    TwoStroke([super::UiCommandShortcutStroke; 2]),
}

impl UiCommandShortcutSequence {
    pub const fn single(stroke: super::UiCommandShortcutStroke) -> Self {
        Self::Single(stroke)
    }

    pub const fn two_stroke(
        first: super::UiCommandShortcutStroke,
        second: super::UiCommandShortcutStroke,
    ) -> Self {
        Self::TwoStroke([first, second])
    }

    pub const fn len(self) -> usize {
        match self {
            Self::Single(_) => 1,
            Self::TwoStroke(_) => 2,
        }
    }

    pub const fn is_empty(self) -> bool {
        false
    }

    pub fn strokes(&self) -> &[super::UiCommandShortcutStroke] {
        match self {
            Self::Single(stroke) => core::slice::from_ref(stroke),
            Self::TwoStroke(strokes) => strokes,
        }
    }

    pub fn format(self, platform: super::UiCommandShortcutPlatform) -> String {
        super::formatting::format_sequence(self, platform)
    }

    pub(crate) fn digest_basis(self) -> String {
        self.strokes()
            .iter()
            .map(|stroke| {
                let key = stroke.key();
                let key_kind = if key.is_physical() {
                    "physical"
                } else {
                    "logical"
                };
                format!(
                    "{key_kind}:{}:{}",
                    key.code().canonical_name(),
                    stroke.modifiers().bits()
                )
            })
            .collect::<Vec<_>>()
            .join("/")
    }

    pub(crate) fn has_conflicting_primary_alias(self) -> bool {
        self.strokes()
            .iter()
            .any(|stroke| stroke.modifiers().has_conflicting_primary_alias())
    }
}
