use super::registry::{registry_row_for_class, ForgeQueryConsumerResidueClass};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryConsumerResidueSourceSite {
    source_label: String,
    source_path: String,
    line: usize,
    column: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryConsumerResidueFinding {
    source_site: ForgeQueryConsumerResidueSourceSite,
    residue_class: ForgeQueryConsumerResidueClass,
    detection_key: &'static str,
    matched_pattern: String,
}

impl ForgeQueryConsumerResidueSourceSite {
    pub(crate) fn new(
        source_label: impl Into<String>,
        source_path: impl Into<String>,
        line: usize,
        column: usize,
    ) -> Self {
        Self {
            source_label: source_label.into(),
            source_path: source_path.into(),
            line,
            column,
        }
    }

    pub fn source_label(&self) -> &str {
        &self.source_label
    }

    pub fn source_path(&self) -> &str {
        &self.source_path
    }

    pub fn line(&self) -> usize {
        self.line
    }

    pub fn column(&self) -> usize {
        self.column
    }
}

impl ForgeQueryConsumerResidueFinding {
    pub(crate) fn discovered(
        source_site: ForgeQueryConsumerResidueSourceSite,
        residue_class: ForgeQueryConsumerResidueClass,
        matched_pattern: impl Into<String>,
    ) -> Self {
        let detection_key = registry_row_for_class(residue_class).detection_key();
        Self {
            source_site,
            residue_class,
            detection_key,
            matched_pattern: matched_pattern.into(),
        }
    }

    pub fn source_site(&self) -> &ForgeQueryConsumerResidueSourceSite {
        &self.source_site
    }

    pub fn source_label(&self) -> &str {
        self.source_site.source_label()
    }

    pub fn source_path(&self) -> &str {
        self.source_site.source_path()
    }

    pub fn line(&self) -> usize {
        self.source_site.line()
    }

    pub fn column(&self) -> usize {
        self.source_site.column()
    }

    pub fn residue_class(&self) -> ForgeQueryConsumerResidueClass {
        self.residue_class
    }

    pub fn detection_key(&self) -> &'static str {
        self.detection_key
    }

    pub fn matched_pattern(&self) -> &str {
        &self.matched_pattern
    }

    pub fn explanation(&self) -> &'static str {
        registry_row_for_class(self.residue_class).explanation()
    }

    pub fn replacement_lane(&self) -> &'static str {
        registry_row_for_class(self.residue_class).replacement_lane()
    }
}
