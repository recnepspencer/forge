#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StoreAccessMode {
    Buffered,
    Mmap,
    DirectIo,
    Mixed,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StoreAccessOperation {
    Read,
    Write,
}
