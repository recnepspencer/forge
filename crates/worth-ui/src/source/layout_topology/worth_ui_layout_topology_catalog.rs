use super::WorthUiLayoutTopologyNode;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiLayoutTopologyCatalog {
    pages: Vec<WorthUiPageLayoutTopology>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiPageLayoutTopology {
    page_name: String,
    layout_name: String,
    root: WorthUiLayoutTopologyNode,
    dynamic_template: bool,
}

impl WorthUiLayoutTopologyCatalog {
    pub fn new(pages: Vec<WorthUiPageLayoutTopology>) -> Self {
        Self { pages }
    }

    pub fn pages(&self) -> &[WorthUiPageLayoutTopology] {
        &self.pages
    }

    pub fn page(&self, page_name: &str) -> Option<&WorthUiPageLayoutTopology> {
        self.pages.iter().find(|page| page.page_name() == page_name)
    }
}

impl WorthUiPageLayoutTopology {
    pub fn new(
        page_name: impl Into<String>,
        layout_name: impl Into<String>,
        root: WorthUiLayoutTopologyNode,
        dynamic_template: bool,
    ) -> Self {
        Self {
            page_name: page_name.into(),
            layout_name: layout_name.into(),
            root,
            dynamic_template,
        }
    }

    pub fn page_name(&self) -> &str {
        &self.page_name
    }

    pub fn layout_name(&self) -> &str {
        &self.layout_name
    }

    pub fn root(&self) -> &WorthUiLayoutTopologyNode {
        &self.root
    }

    pub fn dynamic_template(&self) -> bool {
        self.dynamic_template
    }
}
