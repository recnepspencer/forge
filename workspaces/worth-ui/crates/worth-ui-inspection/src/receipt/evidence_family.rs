#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[non_exhaustive]
pub enum UiEvidenceFamily {
    Declaration,
    Admission,
    Graph,
    Aspect,
    Obligation,
}
