#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiPrimitiveSourceSpan {
    start_byte: usize,
    end_byte: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiPrimitiveDenialPresentation {
    title: &'static str,
    rows: Vec<WorthUiPrimitiveDenialPresentationRow>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiPrimitiveDenialPresentationRow {
    label: &'static str,
    value: String,
}

impl WorthUiPrimitiveSourceSpan {
    pub(crate) fn new(start_byte: usize, end_byte: usize) -> Self {
        Self {
            start_byte,
            end_byte,
        }
    }

    pub fn start_byte(self) -> usize {
        self.start_byte
    }

    pub fn end_byte(self) -> usize {
        self.end_byte
    }
}

impl WorthUiPrimitiveDenialPresentation {
    pub(crate) fn new(title: &'static str, rows: Vec<(&'static str, String)>) -> Self {
        Self {
            title,
            rows: rows
                .into_iter()
                .map(|(label, value)| WorthUiPrimitiveDenialPresentationRow { label, value })
                .collect(),
        }
    }

    pub fn title(&self) -> &'static str {
        self.title
    }

    pub fn rows(&self) -> &[WorthUiPrimitiveDenialPresentationRow] {
        &self.rows
    }
}

impl WorthUiPrimitiveDenialPresentationRow {
    pub fn label(&self) -> &'static str {
        self.label
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}
