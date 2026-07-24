#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UiHostPresentationCostReport {
    presented_surfaces: u64,
    translated_rows: u64,
    translated_bytes: u64,
    native_resource_cache_hits: u64,
    native_resource_cache_misses: u64,
    asynchronous_handoffs: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiHostPresentationCostInput {
    pub presented_surfaces: u64,
    pub translated_rows: u64,
    pub translated_bytes: u64,
    pub native_resource_cache_hits: u64,
    pub native_resource_cache_misses: u64,
    pub asynchronous_handoffs: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiHostPresentationCostOverflow;

impl UiHostPresentationCostReport {
    pub const fn from_adapter(input: UiHostPresentationCostInput) -> Self {
        Self {
            presented_surfaces: input.presented_surfaces,
            translated_rows: input.translated_rows,
            translated_bytes: input.translated_bytes,
            native_resource_cache_hits: input.native_resource_cache_hits,
            native_resource_cache_misses: input.native_resource_cache_misses,
            asynchronous_handoffs: input.asynchronous_handoffs,
        }
    }

    pub fn checked_add(self, other: Self) -> Result<Self, UiHostPresentationCostOverflow> {
        Ok(Self {
            presented_surfaces: checked(self.presented_surfaces, other.presented_surfaces)?,
            translated_rows: checked(self.translated_rows, other.translated_rows)?,
            translated_bytes: checked(self.translated_bytes, other.translated_bytes)?,
            native_resource_cache_hits: checked(
                self.native_resource_cache_hits,
                other.native_resource_cache_hits,
            )?,
            native_resource_cache_misses: checked(
                self.native_resource_cache_misses,
                other.native_resource_cache_misses,
            )?,
            asynchronous_handoffs: checked(
                self.asynchronous_handoffs,
                other.asynchronous_handoffs,
            )?,
        })
    }

    pub const fn presented_surfaces(self) -> u64 {
        self.presented_surfaces
    }

    pub const fn translated_rows(self) -> u64 {
        self.translated_rows
    }

    pub const fn translated_bytes(self) -> u64 {
        self.translated_bytes
    }

    pub const fn native_resource_cache_hits(self) -> u64 {
        self.native_resource_cache_hits
    }

    pub const fn native_resource_cache_misses(self) -> u64 {
        self.native_resource_cache_misses
    }

    pub const fn asynchronous_handoffs(self) -> u64 {
        self.asynchronous_handoffs
    }
}

fn checked(left: u64, right: u64) -> Result<u64, UiHostPresentationCostOverflow> {
    left.checked_add(right)
        .ok_or(UiHostPresentationCostOverflow)
}
