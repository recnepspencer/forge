use worth_store_physical_format::{
    PageGenerationCell, PhysicalGeneration, PhysicalGenerationAuthority, PhysicalPageId,
    PhysicalSegmentId,
};

pub fn page_generation(generation_value: u64, page_value: u64) -> PageGenerationCell {
    PhysicalGenerationAuthority::for_canonical_physical_format()
        .page_cell(segment(1), page(page_value))
        .with_page_generation(generation(generation_value))
}

pub fn segment(value: u64) -> PhysicalSegmentId {
    PhysicalSegmentId::from_raw(value).unwrap()
}

pub fn page(value: u64) -> PhysicalPageId {
    PhysicalPageId::from_raw(value).unwrap()
}

pub fn generation(value: u64) -> PhysicalGeneration {
    PhysicalGeneration::from_raw(value).unwrap()
}
