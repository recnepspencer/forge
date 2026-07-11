#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phase28LayoutAuthorityPosture {
    TerminalOnly,
    ReadmissionRequired,
    ReadmittedEvidence,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdmittedExportBundleLayoutRule {
    _private: (),
}

impl AdmittedExportBundleLayoutRule {
    pub(crate) const fn internal_phase28() -> Self {
        Self { _private: () }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdmittedCapsuleManifestLayoutRule {
    _private: (),
}

impl AdmittedCapsuleManifestLayoutRule {
    pub(crate) const fn internal_phase28() -> Self {
        Self { _private: () }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdmittedRestoreEvidenceLayoutRule {
    _private: (),
}

impl AdmittedRestoreEvidenceLayoutRule {
    pub(crate) const fn internal_phase28() -> Self {
        Self { _private: () }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdmittedImportReadmissionLayoutRule {
    _private: (),
}

impl AdmittedImportReadmissionLayoutRule {
    pub(crate) const fn internal_phase28() -> Self {
        Self { _private: () }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdmittedOfflineVerifierLayoutRule {
    _private: (),
}

impl AdmittedOfflineVerifierLayoutRule {
    pub(crate) const fn internal_phase28() -> Self {
        Self { _private: () }
    }
}
