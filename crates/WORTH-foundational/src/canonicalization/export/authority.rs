use worth_proof::AuthorityMarker;

#[derive(Debug, PartialEq, Eq)]
pub struct CanonicalExportReadmissionAuthority(());

impl CanonicalExportReadmissionAuthority {
    #[cfg(test)]
    pub(crate) const fn new() -> Self {
        Self(())
    }
}

impl AuthorityMarker for CanonicalExportReadmissionAuthority {}
