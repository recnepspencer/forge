use std::io;
#[cfg(test)]
use std::path::PathBuf;

use worth_store_physical_format::store_namespace::{
    StagedNamespaceName, StoreNamespaceRelativeRole,
};

use super::MediaPathRole;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NamespaceConfinementDenialKind {
    EmptyPath,
    AbsolutePath,
    DevicePath,
    ParentTraversal,
    SpecialComponent,
    EmbeddedSeparator,
    AlternateDataStream,
    ReservedDeviceName,
    NonPortableComponent,
    LinkLikeEntry,
    EntryTypeMismatch,
    RootIdentityChanged,
    AuthorityMismatch,
    AuthorityIdentityUnavailable,
    MissingParentPublicationBoundary,
    NamespaceNotAdmissible,
    NamespaceIncomplete,
    NamespaceDamaged,
    NamespaceAmbiguous,
    NamespaceVersionUnsupported,
    OsDenied,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NamespaceConfinementDenial {
    kind: NamespaceConfinementDenialKind,
    io_kind: Option<io::ErrorKind>,
    raw_os_error: Option<i32>,
}

#[cfg(feature = "certification-test-authority")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CertificationConfinementEffect {
    ComponentDenied(NamespaceConfinementDenial),
    OpenDenied(super::MediaEffectStatus),
    WriteReached(super::MediaEffectStatus),
}

impl NamespaceConfinementDenial {
    pub const fn kind(self) -> NamespaceConfinementDenialKind {
        self.kind
    }

    pub const fn io_kind(self) -> Option<io::ErrorKind> {
        self.io_kind
    }

    pub const fn raw_os_error(self) -> Option<i32> {
        self.raw_os_error
    }

    pub(super) const fn structural(kind: NamespaceConfinementDenialKind) -> Self {
        Self {
            kind,
            io_kind: None,
            raw_os_error: None,
        }
    }

    pub(super) fn from_io(error: &io::Error) -> Self {
        Self {
            kind: NamespaceConfinementDenialKind::OsDenied,
            io_kind: Some(error.kind()),
            raw_os_error: error.raw_os_error(),
        }
    }
}

/// A namespace-relative file capability minted by one admitted media owner.
///
/// The owner identity is part of the value. Equal relative names minted by
/// different owners are deliberately not interchangeable.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NamespaceRelativePath {
    owner: super::MediaOwnerIdentity,
    parent: NamespaceParent,
    file_name: String,
    role: MediaPathRole,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum NamespaceParent {
    Root,
    Namespace,
    Families,
    Staging,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StagedNamespacePath(NamespaceRelativePath);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NamespacePublicationTarget(NamespaceRelativePath);

impl NamespacePublicationTarget {
    pub(super) fn new(path: NamespaceRelativePath) -> Self {
        Self(path)
    }

    pub(super) fn into_relative(self) -> NamespaceRelativePath {
        self.0
    }
}

impl StagedNamespacePath {
    pub(super) fn new(path: NamespaceRelativePath) -> Self {
        Self(path)
    }

    pub const fn as_relative(&self) -> &NamespaceRelativePath {
        &self.0
    }

    pub(super) fn into_relative(self) -> NamespaceRelativePath {
        self.0
    }
}

impl std::ops::Deref for StagedNamespacePath {
    type Target = NamespaceRelativePath;

    fn deref(&self) -> &Self::Target {
        self.as_relative()
    }
}

impl NamespaceRelativePath {
    pub const fn role(&self) -> MediaPathRole {
        self.role
    }

    pub const fn owner_identity(&self) -> super::MediaOwnerIdentity {
        self.owner
    }

    #[cfg(test)]
    pub(super) fn as_path(&self) -> PathBuf {
        match self.parent {
            NamespaceParent::Root => PathBuf::from(&self.file_name),
            NamespaceParent::Namespace => PathBuf::from("namespace").join(&self.file_name),
            NamespaceParent::Families => PathBuf::from("families").join(&self.file_name),
            NamespaceParent::Staging => PathBuf::from("staging").join(&self.file_name),
        }
    }

    pub(super) const fn parent(&self) -> NamespaceParent {
        self.parent
    }

    pub(super) fn file_name(&self) -> &str {
        &self.file_name
    }

    pub(super) fn bind_role(
        owner: super::MediaOwnerIdentity,
        role: StoreNamespaceRelativeRole,
    ) -> Self {
        let components = role.components();
        let (parent, file_name) = match components {
            [file_name] => (NamespaceParent::Root, *file_name),
            ["namespace", file_name] => (NamespaceParent::Namespace, *file_name),
            ["families", file_name] => (NamespaceParent::Families, *file_name),
            ["staging", file_name] => (NamespaceParent::Staging, *file_name),
            _ => panic!("stable namespace role has unsupported parent topology"),
        };
        validate_component(file_name).expect("stable namespace role must be portable");
        Self {
            owner,
            parent,
            file_name: file_name.to_owned(),
            role: MediaPathRole::Namespace(role),
        }
    }

    pub(super) fn bind_staged_identity(
        owner: super::MediaOwnerIdentity,
        name: &StagedNamespaceName,
    ) -> Self {
        validate_component(name.as_str()).expect("format-owned staged name must be portable");
        Self {
            owner,
            parent: NamespaceParent::Namespace,
            file_name: name.as_str().to_owned(),
            role: MediaPathRole::Namespace(StoreNamespaceRelativeRole::IdentityRecord),
        }
    }

    #[cfg(feature = "certification-test-authority")]
    pub(super) fn bind_certification_staging(
        owner: super::MediaOwnerIdentity,
        component: &str,
    ) -> Result<Self, NamespaceConfinementDenial> {
        validate_component(component)?;
        Ok(Self {
            owner,
            parent: NamespaceParent::Staging,
            file_name: component.to_owned(),
            role: MediaPathRole::ArtifactOwned,
        })
    }
}

fn validate_component(component: &str) -> Result<(), NamespaceConfinementDenial> {
    let denial = |kind| Err(NamespaceConfinementDenial::structural(kind));
    if component.is_empty() {
        return denial(NamespaceConfinementDenialKind::EmptyPath);
    }
    if component == "." || component == ".." {
        return denial(NamespaceConfinementDenialKind::SpecialComponent);
    }
    if component.contains(['/', '\\']) {
        return denial(NamespaceConfinementDenialKind::EmbeddedSeparator);
    }
    if component.contains(':') {
        return denial(NamespaceConfinementDenialKind::AlternateDataStream);
    }
    if component.ends_with(['.', ' ']) {
        return denial(NamespaceConfinementDenialKind::NonPortableComponent);
    }
    if !component
        .bytes()
        .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || b"-_.".contains(&byte))
    {
        return denial(NamespaceConfinementDenialKind::NonPortableComponent);
    }
    let device_stem = component.split('.').next().unwrap_or(component);
    if is_reserved_device_name(device_stem) {
        return denial(NamespaceConfinementDenialKind::ReservedDeviceName);
    }
    Ok(())
}

#[cfg(feature = "certification-test-authority")]
pub(super) fn certification_probe_component(
    component: &str,
) -> Result<(), NamespaceConfinementDenial> {
    validate_component(component)
}

fn is_reserved_device_name(stem: &str) -> bool {
    matches!(stem, "con" | "prn" | "aux" | "nul")
        || matches!(
            stem.strip_prefix("com"),
            Some("1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9")
        )
        || matches!(
            stem.strip_prefix("lpt"),
            Some("1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9")
        )
}
