//! Move-only live-layout pin authority and copyable diagnostic snapshots.

use std::collections::HashSet;

use worth_ui_host_contract::UiGlyphRasterKey;
use worth_ui_host_contract::UiQualifiedTextLayoutIdentity;

use super::key::UiAtlasEntryIdentity;
use super::ownership::{AtlasCore, PinIdentity};
use super::recovery::UiNativeTextAtlasDenial;
use super::recovery::UiNativeTextAtlasGeneration;
use super::transaction::{UiNativeTextAtlasDemand, UiNativeTextAtlasPinTransition};

pub struct UiNativeTextAtlasPin {
    layout: UiQualifiedTextLayoutIdentity,
    entry: UiAtlasEntryIdentity,
    generation: UiNativeTextAtlasGeneration,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiNativeTextAtlasPinSnapshot {
    layout: UiQualifiedTextLayoutIdentity,
    entry: UiAtlasEntryIdentity,
    generation: UiNativeTextAtlasGeneration,
}

impl UiNativeTextAtlasPin {
    #[allow(dead_code, reason = "reserved for native atlas effect ownership")]
    pub(crate) const fn from_native_host(
        layout: UiQualifiedTextLayoutIdentity,
        entry: UiAtlasEntryIdentity,
        generation: UiNativeTextAtlasGeneration,
    ) -> Self {
        Self {
            layout,
            entry,
            generation,
        }
    }

    pub const fn layout(&self) -> UiQualifiedTextLayoutIdentity {
        self.layout
    }

    pub const fn entry(&self) -> UiAtlasEntryIdentity {
        self.entry
    }

    pub const fn generation(&self) -> UiNativeTextAtlasGeneration {
        self.generation
    }

    pub fn snapshot(&self) -> UiNativeTextAtlasPinSnapshot {
        UiNativeTextAtlasPinSnapshot {
            layout: self.layout,
            entry: self.entry,
            generation: self.generation,
        }
    }
}

impl UiNativeTextAtlasPinSnapshot {
    pub const fn layout(self) -> UiQualifiedTextLayoutIdentity {
        self.layout
    }
    pub const fn entry(self) -> UiAtlasEntryIdentity {
        self.entry
    }
    pub const fn generation(self) -> UiNativeTextAtlasGeneration {
        self.generation
    }
}

pub(crate) fn protected_keys(
    core: &AtlasCore,
    transition: &UiNativeTextAtlasPinTransition,
    demands: &[UiNativeTextAtlasDemand],
) -> HashSet<UiGlyphRasterKey> {
    let mut keys = demands
        .iter()
        .map(|demand| demand.key())
        .collect::<HashSet<_>>();
    let released = transition
        .releases()
        .iter()
        .map(|release| PinIdentity::new(release.layout(), release.key()))
        .collect::<HashSet<_>>();
    for pin in core.pins.iter().filter(|pin| !released.contains(pin)) {
        if let Some(key) = core
            .alpha
            .entries
            .keys()
            .chain(core.color.entries.keys())
            .copied()
            .find(|key| pin.key_matches(*key))
        {
            keys.insert(key);
        }
    }
    for addition in transition.additions() {
        keys.insert(addition.key());
    }
    keys
}

pub(crate) fn validate_pin_transition(
    core: &AtlasCore,
    transition: &UiNativeTextAtlasPinTransition,
) -> Result<(), UiNativeTextAtlasDenial> {
    let mut seen = HashSet::new();
    for release in transition.releases() {
        if !seen.insert(PinIdentity::new(release.layout(), release.key()))
            || !core
                .pins
                .contains(&PinIdentity::new(release.layout(), release.key()))
        {
            return Err(UiNativeTextAtlasDenial::StalePin);
        }
    }
    for add in transition.additions() {
        if !seen.insert(PinIdentity::new(add.layout(), add.key()))
            || core
                .pins
                .contains(&PinIdentity::new(add.layout(), add.key()))
        {
            return Err(UiNativeTextAtlasDenial::PinConflict);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pin_authority_is_not_copy_and_snapshot_is() {
        let pin = UiNativeTextAtlasPin::from_native_host(
            UiQualifiedTextLayoutIdentity::from_text_mechanics([1; 32]),
            UiAtlasEntryIdentity::from_native_host(3).unwrap(),
            UiNativeTextAtlasGeneration::new(1).unwrap(),
        );
        let snapshot = pin.snapshot();
        let copied = snapshot;
        assert_eq!(copied.entry().get(), 3);
        assert_eq!(pin.layout().digest(), [1; 32]);
        let _moved = pin;
    }
}
