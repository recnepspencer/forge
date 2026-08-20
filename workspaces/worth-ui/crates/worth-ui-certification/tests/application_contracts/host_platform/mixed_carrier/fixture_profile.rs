#[derive(Clone, Copy)]
pub(crate) struct MixedCarrierFixtureProfile {
    pub(crate) rectangle_count: usize,
    pub(crate) rectangle_component_count: usize,
    pub(crate) scalar_instance_count: usize,
    pub(crate) text_count: usize,
    pub(crate) collection_rows: usize,
    pub(crate) text_bytes: usize,
    pub(crate) ordinary_text_bytes: usize,
    pub(crate) final_text_bytes: usize,
}

pub(crate) const SMOKE: MixedCarrierFixtureProfile = MixedCarrierFixtureProfile {
    rectangle_count: 8,
    rectangle_component_count: 5,
    scalar_instance_count: 4,
    text_count: 8,
    collection_rows: 3,
    text_bytes: 238,
    ordinary_text_bytes: 17,
    final_text_bytes: 13,
};

pub(crate) const CLOSURE: MixedCarrierFixtureProfile = MixedCarrierFixtureProfile {
    rectangle_count: 2_048,
    rectangle_component_count: 1_361,
    scalar_instance_count: 688,
    text_count: 2_048,
    collection_rows: 1_359,
    text_bytes: 1_048_576,
    ordinary_text_bytes: 729,
    final_text_bytes: 675,
};
