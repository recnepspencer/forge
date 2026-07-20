use std::collections::BTreeSet;

use crate::runtime::WorthQueryAspectTouch;

pub(super) fn insert_declared_aspects(
    changed_aspects: &mut BTreeSet<WorthQueryAspectTouch>,
    aspects: &[WorthQueryAspectTouch],
) {
    for touch in aspects {
        changed_aspects.insert(touch.clone());
    }
}

pub(super) fn aspects_match(
    declared_aspects: &[WorthQueryAspectTouch],
    changed_aspect: &WorthQueryAspectTouch,
) -> bool {
    declared_aspects
        .iter()
        .any(|declared| declared.matches_or_contains(changed_aspect))
}
