#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthUiCapabilityReloadFamilyCounters {
    source_parse_count: usize,
    canonicalization_count: usize,
    edited_delta_width: usize,
    changed_descriptor_count: usize,
    family_rebuild_breadth: usize,
    registry_lookup_count: usize,
}

impl WorthUiCapabilityReloadFamilyCounters {
    pub(crate) fn new(
        source_parse_count: usize,
        canonicalization_count: usize,
        edited_delta_width: usize,
        changed_descriptor_count: usize,
        family_rebuild_breadth: usize,
        registry_lookup_count: usize,
    ) -> Self {
        Self {
            source_parse_count,
            canonicalization_count,
            edited_delta_width,
            changed_descriptor_count,
            family_rebuild_breadth,
            registry_lookup_count,
        }
    }

    pub(crate) fn add(self, other: Self) -> Self {
        Self {
            source_parse_count: self.source_parse_count + other.source_parse_count,
            canonicalization_count: self.canonicalization_count + other.canonicalization_count,
            edited_delta_width: self.edited_delta_width + other.edited_delta_width,
            changed_descriptor_count: self.changed_descriptor_count
                + other.changed_descriptor_count,
            family_rebuild_breadth: self.family_rebuild_breadth + other.family_rebuild_breadth,
            registry_lookup_count: self.registry_lookup_count + other.registry_lookup_count,
        }
    }

    pub fn source_parse_count(self) -> usize {
        self.source_parse_count
    }

    pub fn canonicalization_count(self) -> usize {
        self.canonicalization_count
    }

    pub fn edited_delta_width(self) -> usize {
        self.edited_delta_width
    }

    pub fn changed_descriptor_count(self) -> usize {
        self.changed_descriptor_count
    }

    pub fn family_rebuild_breadth(self) -> usize {
        self.family_rebuild_breadth
    }

    pub fn registry_lookup_count(self) -> usize {
        self.registry_lookup_count
    }

    pub fn descriptor_lookup_count(self) -> usize {
        self.registry_lookup_count
    }
}
