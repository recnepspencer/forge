#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AbsenceLaw {
    Required,
    Optional,
    Defaulted,
}
