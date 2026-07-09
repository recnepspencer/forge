use std::collections::BTreeSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ForbiddenShortcutKind {
    LogsAsProof,
    JsonScenarioAuthority,
    TerminalProjectionAuthority,
    SameRunSelfComparison,
    PrivateMutation,
    FixtureLabelAuthority,
    CopiedDigestAuthority,
    SkippedProofProgression,
    TestSupportVerdictAuthority,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForbiddenShortcutSet {
    shortcuts: BTreeSet<ForbiddenShortcutKind>,
}

impl ForbiddenShortcutSet {
    pub fn empty() -> Self {
        Self {
            shortcuts: BTreeSet::new(),
        }
    }

    pub fn roadmap2_baseline() -> Self {
        Self::from_shortcuts(ROADMAP_2_BASELINE_SHORTCUTS)
    }

    pub fn without(mut self, shortcut: ForbiddenShortcutKind) -> Self {
        self.shortcuts.remove(&shortcut);
        self
    }

    pub fn contains(&self, shortcut: ForbiddenShortcutKind) -> bool {
        self.shortcuts.contains(&shortcut)
    }

    pub fn iter(&self) -> impl Iterator<Item = ForbiddenShortcutKind> + '_ {
        self.shortcuts.iter().copied()
    }

    pub(crate) fn from_shortcuts(
        shortcuts: impl IntoIterator<Item = ForbiddenShortcutKind>,
    ) -> Self {
        Self {
            shortcuts: shortcuts.into_iter().collect(),
        }
    }
}

pub(crate) const ROADMAP_2_BASELINE_SHORTCUTS: [ForbiddenShortcutKind; 9] = [
    ForbiddenShortcutKind::LogsAsProof,
    ForbiddenShortcutKind::JsonScenarioAuthority,
    ForbiddenShortcutKind::TerminalProjectionAuthority,
    ForbiddenShortcutKind::SameRunSelfComparison,
    ForbiddenShortcutKind::PrivateMutation,
    ForbiddenShortcutKind::FixtureLabelAuthority,
    ForbiddenShortcutKind::CopiedDigestAuthority,
    ForbiddenShortcutKind::SkippedProofProgression,
    ForbiddenShortcutKind::TestSupportVerdictAuthority,
];

pub(crate) fn forbidden_shortcut_token(shortcut: ForbiddenShortcutKind) -> &'static str {
    match shortcut {
        ForbiddenShortcutKind::LogsAsProof => "logs-as-proof",
        ForbiddenShortcutKind::JsonScenarioAuthority => "json-scenario-authority",
        ForbiddenShortcutKind::TerminalProjectionAuthority => "terminal-projection-authority",
        ForbiddenShortcutKind::SameRunSelfComparison => "same-run-self-comparison",
        ForbiddenShortcutKind::PrivateMutation => "private-mutation",
        ForbiddenShortcutKind::FixtureLabelAuthority => "fixture-label-authority",
        ForbiddenShortcutKind::CopiedDigestAuthority => "copied-digest-authority",
        ForbiddenShortcutKind::SkippedProofProgression => "skipped-proof-progression",
        ForbiddenShortcutKind::TestSupportVerdictAuthority => "test-support-verdict-authority",
    }
}
