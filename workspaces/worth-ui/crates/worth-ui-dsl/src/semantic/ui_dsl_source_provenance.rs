#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UiDslSourceProvenance {
    FileAuthored {
        module_path: String,
        declaration_index: usize,
    },
    RustAuthored {
        module_path: String,
        declaration_index: usize,
    },
}

impl UiDslSourceProvenance {
    pub fn file_authored(module_path: impl Into<String>, declaration_index: usize) -> Self {
        Self::FileAuthored {
            module_path: module_path.into(),
            declaration_index,
        }
    }

    pub fn rust_authored(module_path: impl Into<String>, declaration_index: usize) -> Self {
        Self::RustAuthored {
            module_path: module_path.into(),
            declaration_index,
        }
    }

    pub fn module_path(&self) -> &str {
        match self {
            Self::FileAuthored { module_path, .. } => module_path,
            Self::RustAuthored { module_path, .. } => module_path,
        }
    }

    pub fn declaration_index(&self) -> usize {
        match self {
            Self::FileAuthored {
                declaration_index, ..
            } => *declaration_index,
            Self::RustAuthored {
                declaration_index, ..
            } => *declaration_index,
        }
    }
}
