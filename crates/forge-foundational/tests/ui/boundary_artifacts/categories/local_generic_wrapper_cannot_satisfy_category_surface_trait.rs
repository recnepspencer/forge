use forge_foundational::{boundary_summary_category_of, FoundationalBoundaryCategorySurface};

struct LocalGenericWrapper<T> {
    payload: T,
}

impl<T> FoundationalBoundaryCategorySurface for LocalGenericWrapper<T> {
    type Category = forge_foundational::SummaryCategory;
}

fn main() {
    let wrapper = LocalGenericWrapper { payload: "summary".to_string() };
    let _ = boundary_summary_category_of(&wrapper);
}
