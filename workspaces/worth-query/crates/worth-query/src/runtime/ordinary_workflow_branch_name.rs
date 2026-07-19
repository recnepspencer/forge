/// Query-owned, declaration-admitted name for a lower-runtime branch.
///
/// Construction remains inside Query so ordinary callers and custom backends
/// cannot forge a branch name after declaration admission. Backend boundaries
/// may inspect the admitted name when translating it into their native branch
/// identity.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryAdmittedBranchName(String);

impl WorthQueryAdmittedBranchName {
    pub(crate) fn admit(authored: impl Into<String>) -> Option<Self> {
        let name = authored.into().trim().to_string();
        (!name.is_empty()).then_some(Self(name))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
