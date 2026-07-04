#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[non_exhaustive]
pub enum UiEvidenceMaterializationPosture {
    RefsOnly,
    SummaryAvailable,
    DetailAvailable,
}
