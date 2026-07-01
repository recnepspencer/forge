use crate::graph::{UiGraphLookupSurface, UiGraphSnapshot};

impl UiGraphSnapshot {
    pub fn lookup(&self) -> UiGraphLookupSurface<'_> {
        UiGraphLookupSurface::new(self)
    }
}
