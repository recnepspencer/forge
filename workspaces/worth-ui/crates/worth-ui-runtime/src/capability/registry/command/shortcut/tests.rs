use super::{
    UiCommandKeyCode, UiCommandModifierSet, UiCommandShortcutPlatform, UiCommandShortcutSequence,
    UiCommandShortcutStroke,
};

#[test]
fn logical_and_physical_keys_are_distinct_identity() {
    let modifiers = UiCommandModifierSet::none().with_primary();
    let logical = UiCommandShortcutSequence::single(UiCommandShortcutStroke::logical(
        UiCommandKeyCode::K,
        modifiers,
    ));
    let physical = UiCommandShortcutSequence::single(UiCommandShortcutStroke::physical(
        UiCommandKeyCode::K,
        modifiers,
    ));

    assert_ne!(logical, physical);
    assert_ne!(logical.digest_basis(), physical.digest_basis());
}

#[test]
fn display_platform_alias_does_not_change_shortcut_identity() {
    let shortcut = UiCommandShortcutSequence::single(UiCommandShortcutStroke::logical(
        UiCommandKeyCode::S,
        UiCommandModifierSet::none().with_primary().with_shift(),
    ));

    assert_eq!(
        shortcut.format(UiCommandShortcutPlatform::MacOs),
        "Cmd+Shift+S"
    );
    assert_eq!(
        shortcut.format(UiCommandShortcutPlatform::Windows),
        "Ctrl+Shift+S"
    );
    assert_eq!(
        shortcut.format(UiCommandShortcutPlatform::Linux),
        "Ctrl+Shift+S"
    );
    assert_eq!(shortcut, shortcut);
}

#[test]
fn ordinary_sequences_are_bounded_to_one_or_two_strokes() {
    let first = UiCommandShortcutStroke::logical(
        UiCommandKeyCode::K,
        UiCommandModifierSet::none().with_primary(),
    );
    let second = UiCommandShortcutStroke::logical(
        UiCommandKeyCode::S,
        UiCommandModifierSet::none().with_primary(),
    );

    assert_eq!(UiCommandShortcutSequence::single(first).len(), 1);
    assert_eq!(
        UiCommandShortcutSequence::two_stroke(first, second).len(),
        2
    );
}
