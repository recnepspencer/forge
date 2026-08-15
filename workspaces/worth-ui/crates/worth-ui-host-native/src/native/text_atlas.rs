//! Inert Phase 5 native atlas lifecycle contract.
//!
//! The native host owns these bounded snapshots and pin identities. This file
//! performs no GPU allocation, upload, eviction, or raster work.

use worth_ui_host_contract::{
    UiQualifiedFontFaceIdentity, UiQualifiedTextLayoutIdentity, UiTextScaleGeneration,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct UiAtlasEntryIdentity(u64);

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct UiAtlasEntryKey {
    face: UiQualifiedFontFaceIdentity,
    glyph_id: u32,
    dpi_milli: u32,
    text_scale: UiTextScaleGeneration,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiTextAtlasCapacity {
    entries: u32,
    bytes: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiAtlasPin {
    layout: UiQualifiedTextLayoutIdentity,
    entry: UiAtlasEntryIdentity,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiTextAtlasContractDenial {
    EntryCapacityExceeded,
    ByteCapacityExceeded,
    PinnedEntryCountExceeded,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct UiTextAtlasLifecycleState {
    capacity: UiTextAtlasCapacity,
    live_entries: u32,
    pinned_entries: u32,
    retained_bytes: u64,
    high_water_entries: u32,
    high_water_bytes: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct UiTextAtlasLifecycleInput {
    capacity: UiTextAtlasCapacity,
    live_entries: u32,
    pinned_entries: u32,
    retained_bytes: u64,
    high_water_entries: u32,
    high_water_bytes: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiAlphaAtlasLifecycle(UiTextAtlasLifecycleState);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiRgbaAtlasLifecycle(UiTextAtlasLifecycleState);

impl UiAtlasEntryIdentity {
    pub(crate) const fn from_native_host(value: u64) -> Option<Self> {
        if value == 0 {
            None
        } else {
            Some(Self(value))
        }
    }

    pub const fn get(self) -> u64 {
        self.0
    }
}

impl UiAtlasEntryKey {
    pub(crate) const fn from_native_host(
        face: UiQualifiedFontFaceIdentity,
        glyph_id: u32,
        dpi_milli: u32,
        text_scale: UiTextScaleGeneration,
    ) -> Option<Self> {
        if dpi_milli == 0 {
            None
        } else {
            Some(Self {
                face,
                glyph_id,
                dpi_milli,
                text_scale,
            })
        }
    }

    pub const fn face(self) -> UiQualifiedFontFaceIdentity {
        self.face
    }
    pub const fn glyph_id(self) -> u32 {
        self.glyph_id
    }
    pub const fn dpi_milli(self) -> u32 {
        self.dpi_milli
    }
    pub const fn text_scale(self) -> UiTextScaleGeneration {
        self.text_scale
    }
}

impl UiTextAtlasCapacity {
    pub const fn new(entries: u32, bytes: u64) -> Option<Self> {
        if entries == 0 || bytes == 0 {
            None
        } else {
            Some(Self { entries, bytes })
        }
    }

    pub const fn entries(self) -> u32 {
        self.entries
    }
    pub const fn bytes(self) -> u64 {
        self.bytes
    }
}

impl UiAtlasPin {
    pub(crate) const fn from_native_host(
        layout: UiQualifiedTextLayoutIdentity,
        entry: UiAtlasEntryIdentity,
    ) -> Self {
        Self { layout, entry }
    }

    pub const fn layout(self) -> UiQualifiedTextLayoutIdentity {
        self.layout
    }
    pub const fn entry(self) -> UiAtlasEntryIdentity {
        self.entry
    }
}

macro_rules! atlas_lifecycle {
    ($type:ty) => {
        impl $type {
            fn from_native_host(
                input: UiTextAtlasLifecycleInput,
            ) -> Result<Self, UiTextAtlasContractDenial> {
                let state = UiTextAtlasLifecycleState::admit(input)?;
                Ok(Self(state))
            }

            pub const fn capacity(self) -> UiTextAtlasCapacity {
                self.0.capacity
            }
            pub const fn live_entries(self) -> u32 {
                self.0.live_entries
            }
            pub const fn pinned_entries(self) -> u32 {
                self.0.pinned_entries
            }
            pub const fn retained_bytes(self) -> u64 {
                self.0.retained_bytes
            }
            pub const fn high_water_entries(self) -> u32 {
                self.0.high_water_entries
            }
            pub const fn high_water_bytes(self) -> u64 {
                self.0.high_water_bytes
            }
        }
    };
}

atlas_lifecycle!(UiAlphaAtlasLifecycle);
atlas_lifecycle!(UiRgbaAtlasLifecycle);

impl UiTextAtlasLifecycleState {
    const fn admit(input: UiTextAtlasLifecycleInput) -> Result<Self, UiTextAtlasContractDenial> {
        if input.live_entries > input.capacity.entries
            || input.high_water_entries > input.capacity.entries
        {
            return Err(UiTextAtlasContractDenial::EntryCapacityExceeded);
        }
        if input.retained_bytes > input.capacity.bytes
            || input.high_water_bytes > input.capacity.bytes
        {
            return Err(UiTextAtlasContractDenial::ByteCapacityExceeded);
        }
        if input.pinned_entries > input.live_entries {
            return Err(UiTextAtlasContractDenial::PinnedEntryCountExceeded);
        }
        Ok(Self {
            capacity: input.capacity,
            live_entries: input.live_entries,
            pinned_entries: input.pinned_entries,
            retained_bytes: input.retained_bytes,
            high_water_entries: input.high_water_entries,
            high_water_bytes: input.high_water_bytes,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn separate_atlas_contracts_reject_over_capacity_and_impossible_pins() {
        let capacity = UiTextAtlasCapacity::new(2, 64).unwrap();
        let layout = UiQualifiedTextLayoutIdentity::from_text_mechanics([1; 32]);
        let entry = UiAtlasEntryIdentity::from_native_host(1).unwrap();
        let pin = UiAtlasPin::from_native_host(layout, entry);
        let key = UiAtlasEntryKey::from_native_host(
            UiQualifiedFontFaceIdentity::from_text_mechanics([2; 32], 0),
            7,
            1_500,
            UiTextScaleGeneration::new(1).unwrap(),
        )
        .unwrap();
        let input = |live_entries, pinned_entries, high_water_entries| UiTextAtlasLifecycleInput {
            capacity,
            live_entries,
            pinned_entries,
            retained_bytes: 64,
            high_water_entries,
            high_water_bytes: 64,
        };
        assert!(UiAlphaAtlasLifecycle::from_native_host(input(2, 1, 2)).is_ok());
        assert_eq!(pin.entry(), entry);
        assert_eq!(key.glyph_id(), 7);
        assert_eq!(
            UiRgbaAtlasLifecycle::from_native_host(input(2, 3, 2)),
            Err(UiTextAtlasContractDenial::PinnedEntryCountExceeded)
        );
        assert_eq!(
            UiAlphaAtlasLifecycle::from_native_host(input(3, 0, 3)),
            Err(UiTextAtlasContractDenial::EntryCapacityExceeded)
        );
    }
}
