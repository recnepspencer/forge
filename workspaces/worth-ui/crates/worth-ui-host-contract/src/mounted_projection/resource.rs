#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiMountedResourceKind {
    Image,
    Font,
    CanvasContract,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiMountedResourceReference(u16);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiMountedResourceEntry {
    content_identity: u64,
    kind: UiMountedResourceKind,
    byte_len: u32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiMountedResourceTable {
    entries: std::sync::Arc<[UiMountedResourceEntry]>,
}

impl UiMountedResourceReference {
    pub fn new(index: u16) -> Self {
        Self(index)
    }
    pub fn index(self) -> u16 {
        self.0
    }
}

impl UiMountedResourceEntry {
    pub fn new(content_identity: u64, kind: UiMountedResourceKind, byte_len: u32) -> Self {
        Self {
            content_identity,
            kind,
            byte_len,
        }
    }
    pub fn content_identity(&self) -> u64 {
        self.content_identity
    }
    pub fn kind(&self) -> UiMountedResourceKind {
        self.kind
    }
    pub fn byte_len(&self) -> u32 {
        self.byte_len
    }
}

impl UiMountedResourceTable {
    pub fn new(entries: Vec<UiMountedResourceEntry>) -> Self {
        Self {
            entries: entries.into(),
        }
    }
    pub fn entries(&self) -> &[UiMountedResourceEntry] {
        &self.entries
    }
    pub fn resolve(
        &self,
        reference: UiMountedResourceReference,
    ) -> Option<&UiMountedResourceEntry> {
        self.entries.get(usize::from(reference.index()))
    }
}
