#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiDeclaredPostureApplicability {
    Required,
    Optional,
    NotApplicable,
    ArchitecturallyOwnedButNotYetAdmitted,
}
