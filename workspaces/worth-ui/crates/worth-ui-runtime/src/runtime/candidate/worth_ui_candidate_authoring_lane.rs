#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiCandidateAuthoringLane {
    FileAuthored,
    RustAuthored,
}

impl WorthUiCandidateAuthoringLane {
    pub fn file_authored() -> Self {
        Self::FileAuthored
    }

    pub fn rust_authored() -> Self {
        Self::RustAuthored
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::FileAuthored => "file-authored",
            Self::RustAuthored => "rust-authored",
        }
    }
}
