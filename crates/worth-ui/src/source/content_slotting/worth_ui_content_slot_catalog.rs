use crate::source::WorthUiPageContentSlots;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiContentSlotCatalog {
    pages: Vec<WorthUiPageContentSlots>,
}

impl WorthUiContentSlotCatalog {
    pub(crate) fn from_prepared_pages(pages: Vec<WorthUiPageContentSlots>) -> Self {
        Self { pages }
    }

    pub fn pages(&self) -> &[WorthUiPageContentSlots] {
        &self.pages
    }

    pub fn page(&self, page_name: &str) -> Option<&WorthUiPageContentSlots> {
        self.pages.iter().find(|page| page.page_name() == page_name)
    }
}
