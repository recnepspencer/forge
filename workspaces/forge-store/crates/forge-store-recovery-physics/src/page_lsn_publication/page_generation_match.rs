use forge_store_physical_format::PageGenerationCell;

pub(crate) const fn same_page_generation(
    left: PageGenerationCell,
    right: PageGenerationCell,
) -> bool {
    left.segment_id().get() == right.segment_id().get()
        && left.page_id().get() == right.page_id().get()
        && left.generation().get() == right.generation().get()
}
