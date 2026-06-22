#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct WorthDocsInvariantEvidence {
    actual_relative_path: Option<String>,
    missing_headings: Vec<String>,
    missing_markdown_fragments: Vec<String>,
    missing_readme_fragments: Vec<String>,
    missing_metadata_entries: Vec<String>,
    missing_directories: Vec<String>,
    ownership_count: Option<usize>,
}

impl WorthDocsInvariantEvidence {
    pub fn actual_relative_path(&self) -> Option<&str> {
        self.actual_relative_path.as_deref()
    }

    pub fn missing_headings(&self) -> &[String] {
        &self.missing_headings
    }

    pub fn missing_markdown_fragments(&self) -> &[String] {
        &self.missing_markdown_fragments
    }

    pub fn missing_readme_fragments(&self) -> &[String] {
        &self.missing_readme_fragments
    }

    pub fn missing_metadata_entries(&self) -> &[String] {
        &self.missing_metadata_entries
    }

    pub fn missing_directories(&self) -> &[String] {
        &self.missing_directories
    }

    pub fn ownership_count(&self) -> Option<usize> {
        self.ownership_count
    }

    pub fn set_actual_relative_path(&mut self, actual_relative_path: impl Into<String>) {
        self.actual_relative_path = Some(actual_relative_path.into());
    }

    pub fn push_missing_heading(&mut self, heading: impl Into<String>) {
        self.missing_headings.push(heading.into());
    }

    pub fn push_missing_markdown_fragment(&mut self, fragment: impl Into<String>) {
        self.missing_markdown_fragments.push(fragment.into());
    }

    pub fn push_missing_readme_fragment(&mut self, fragment: impl Into<String>) {
        self.missing_readme_fragments.push(fragment.into());
    }

    pub fn push_missing_metadata_entry(&mut self, entry: impl Into<String>) {
        self.missing_metadata_entries.push(entry.into());
    }

    pub fn push_missing_directory(&mut self, directory: impl Into<String>) {
        self.missing_directories.push(directory.into());
    }

    pub fn set_ownership_count(&mut self, ownership_count: usize) {
        self.ownership_count = Some(ownership_count);
    }

    pub fn first_problem(&self) -> Option<String> {
        self.missing_headings
            .first()
            .map(|heading| format!("required heading `{heading}` is missing"))
            .or_else(|| {
                self.missing_markdown_fragments
                    .first()
                    .map(|fragment| format!("required markdown fragment `{fragment}` is missing"))
            })
            .or_else(|| {
                self.missing_readme_fragments
                    .first()
                    .map(|fragment| format!("required README fragment `{fragment}` is missing"))
            })
            .or_else(|| {
                self.missing_metadata_entries
                    .first()
                    .map(|entry| format!("required metadata entry `{entry}` is missing or drifted"))
            })
            .or_else(|| {
                self.missing_directories
                    .first()
                    .map(|directory| format!("required directory `{directory}` is missing"))
            })
            .or_else(|| {
                self.ownership_count
                    .filter(|count| *count != 1)
                    .map(|count| format!("ownership count drifted to `{count}`"))
            })
            .or_else(|| {
                self.actual_relative_path
                    .as_ref()
                    .map(|path| format!("relative path drifted to `{path}`"))
            })
    }
}
