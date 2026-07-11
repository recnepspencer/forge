use forge_store_physical_certification::layout_harness::scenario::canonical_s8_layout_shortcut_denials;
use forge_store_physical_certification::layout_harness::shortcut_denials::S8LayoutShortcutDenialKind;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S8LayoutAdversarialInputs {
    denied_shortcuts: &'static [S8LayoutShortcutDenialKind],
}

pub fn s8_layout_adversarial_inputs() -> S8LayoutAdversarialInputs {
    S8LayoutAdversarialInputs {
        denied_shortcuts: canonical_s8_layout_shortcut_denials(),
    }
}

impl S8LayoutAdversarialInputs {
    pub const fn denied_shortcuts(&self) -> &'static [S8LayoutShortcutDenialKind] {
        self.denied_shortcuts
    }
}
