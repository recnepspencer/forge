use std::collections::BTreeSet;

use crate::runtime::ForgeQueryAspectTouch;

pub(super) fn validate_declared_effect_aspects(
    aspects: &[ForgeQueryAspectTouch],
) -> Result<(), String> {
    for aspect in aspects {
        if aspect.native_aspect_key().as_str().is_empty() {
            return Err("effect aspect declaration may not be empty".to_string());
        }
    }
    Ok(())
}

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
