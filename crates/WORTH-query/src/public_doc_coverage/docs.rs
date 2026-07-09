#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthQueryPublicDocReference {
    path: &'static str,
    section: &'static str,
}

impl WorthQueryPublicDocReference {
    pub(crate) const fn new(path: &'static str, section: &'static str) -> Self {
        Self { path, section }
    }

    pub fn path(&self) -> &'static str {
        self.path
    }

    pub fn section(&self) -> &'static str {
        self.section
    }
}
