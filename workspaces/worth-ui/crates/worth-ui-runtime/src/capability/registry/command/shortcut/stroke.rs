#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct UiCommandShortcutStroke {
    key: super::UiCommandShortcutKey,
    modifiers: super::UiCommandModifierSet,
}

impl UiCommandShortcutStroke {
    pub const fn new(
        key: super::UiCommandShortcutKey,
        modifiers: super::UiCommandModifierSet,
    ) -> Self {
        Self { key, modifiers }
    }

    pub const fn logical(
        key: super::UiCommandKeyCode,
        modifiers: super::UiCommandModifierSet,
    ) -> Self {
        Self::new(super::UiCommandShortcutKey::logical(key), modifiers)
    }

    pub const fn physical(
        key: super::UiCommandKeyCode,
        modifiers: super::UiCommandModifierSet,
    ) -> Self {
        Self::new(super::UiCommandShortcutKey::physical(key), modifiers)
    }

    pub const fn key(self) -> super::UiCommandShortcutKey {
        self.key
    }

    pub const fn modifiers(self) -> super::UiCommandModifierSet {
        self.modifiers
    }

    pub(crate) const fn resolved_for(self, platform: super::UiCommandShortcutPlatform) -> Self {
        Self::new(self.key, self.modifiers.resolved_for(platform))
    }
}
