#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum UiEvidenceFamily {
    Declaration,
    Admission,
    Graph,
    Aspect,
    Obligation,
}
