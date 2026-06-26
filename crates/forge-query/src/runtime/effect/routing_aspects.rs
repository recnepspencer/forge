use std::collections::BTreeSet;

use crate::runtime::ForgeQueryAspectTouch;

pub(super) fn insert_declared_aspects(
    changed_aspects: &mut BTreeSet<ForgeQueryAspectTouch>,
    aspects: &[ForgeQueryAspectTouch],
) {
    for touch in aspects {
        changed_aspects.insert(touch.clone());
    }
}

pub(super) fn aspects_match(
    declared_aspects: &[ForgeQueryAspectTouch],
    changed_aspect: &ForgeQueryAspectTouch,
) -> bool {
    declared_aspects
        .iter()
        .any(|declared| declared.matches_or_contains(changed_aspect))
}
