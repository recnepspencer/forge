use std::{collections::BTreeSet, sync::Arc};

mod constructors;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum WorthUiProjectionShape {
    Scalar,
    Collection,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum WorthUiProjectionNativeFamily {
    Text,
    Boolean,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum WorthUiProjectionLifecycle {
    Snapshot,
    Live,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct WorthUiProjectionCollectionPolicy {
    requires_complete_result: bool,
    permits_continuation: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiProjectionCollectionSelection {
    selected_fields: Vec<String>,
    lifecycle: WorthUiProjectionLifecycle,
    policy: WorthUiProjectionCollectionPolicy,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct WorthUiProjectionRequirementIdentity(u64);

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct WorthUiProjectionRequirement {
    declaration_identity: Arc<str>,
    view_identity: Arc<str>,
    shape: WorthUiProjectionShape,
    selected_fields: Box<[Arc<str>]>,
    row_identity_field: Option<Arc<str>>,
    native_family: WorthUiProjectionNativeFamily,
    lifecycle: WorthUiProjectionLifecycle,
    collection_policy: Option<WorthUiProjectionCollectionPolicy>,
    identity: WorthUiProjectionRequirementIdentity,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiProjectionDeclarationErrorKind {
    EmptyIdentity,
    DuplicateClause,
    MissingClause,
    UnknownClause,
    UnsupportedValue,
    ScalarFieldCount,
    CollectionFieldCount,
    DuplicateSelectedField,
    ShapeClauseMismatch,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiProjectionDeclarationError {
    kind: WorthUiProjectionDeclarationErrorKind,
    detail: Arc<str>,
}

pub(super) struct WorthUiProjectionRequirementParts {
    pub(super) declaration_identity: String,
    pub(super) view_identity: String,
    pub(super) shape: WorthUiProjectionShape,
    pub(super) selected_fields: Vec<String>,
    pub(super) row_identity_field: Option<String>,
    pub(super) native_family: WorthUiProjectionNativeFamily,
    pub(super) lifecycle: WorthUiProjectionLifecycle,
    pub(super) collection_policy: Option<WorthUiProjectionCollectionPolicy>,
}

impl WorthUiProjectionRequirement {
    pub(super) fn build(
        mut parts: WorthUiProjectionRequirementParts,
    ) -> Result<Self, WorthUiProjectionDeclarationError> {
        validate_requirement_parts(&parts)?;
        parts.selected_fields.sort();
        let identity = identity_for(&parts);
        let selected_fields = parts
            .selected_fields
            .into_iter()
            .map(Arc::<str>::from)
            .collect::<Box<[_]>>();
        let row_identity_field = parts.row_identity_field.map(Arc::<str>::from);
        Ok(Self {
            declaration_identity: Arc::from(parts.declaration_identity),
            view_identity: Arc::from(parts.view_identity),
            shape: parts.shape,
            selected_fields,
            row_identity_field,
            native_family: parts.native_family,
            lifecycle: parts.lifecycle,
            collection_policy: parts.collection_policy,
            identity,
        })
    }

    pub fn declaration_identity(&self) -> &str {
        self.declaration_identity.as_ref()
    }

    pub fn view_identity(&self) -> &str {
        self.view_identity.as_ref()
    }

    pub fn shape(&self) -> WorthUiProjectionShape {
        self.shape
    }

    pub fn selected_fields(&self) -> impl ExactSizeIterator<Item = &str> {
        self.selected_fields.iter().map(AsRef::as_ref)
    }

    pub fn row_identity_field(&self) -> Option<&str> {
        self.row_identity_field.as_deref()
    }

    pub fn native_family(&self) -> WorthUiProjectionNativeFamily {
        self.native_family
    }

    pub fn lifecycle(&self) -> WorthUiProjectionLifecycle {
        self.lifecycle
    }

    pub fn collection_policy(&self) -> Option<WorthUiProjectionCollectionPolicy> {
        self.collection_policy
    }

    pub fn identity(&self) -> WorthUiProjectionRequirementIdentity {
        self.identity
    }
}

fn validate_requirement_parts(
    parts: &WorthUiProjectionRequirementParts,
) -> Result<(), WorthUiProjectionDeclarationError> {
    validate_identity("declaration", &parts.declaration_identity)?;
    validate_identity("view", &parts.view_identity)?;
    validate_selected_fields(&parts.selected_fields)?;
    if let Some(field) = parts.row_identity_field.as_deref() {
        validate_identity("row identity field", field)?;
    }
    match parts.shape {
        WorthUiProjectionShape::Scalar if parts.selected_fields.len() != 1 => Err(error(
            WorthUiProjectionDeclarationErrorKind::ScalarFieldCount,
            "scalar projection requires exactly one selected field",
        )),
        WorthUiProjectionShape::Scalar
            if parts.row_identity_field.is_some() || parts.collection_policy.is_some() =>
        {
            Err(error(
                WorthUiProjectionDeclarationErrorKind::ShapeClauseMismatch,
                "scalar projection cannot declare row or collection policy",
            ))
        }
        WorthUiProjectionShape::Collection if parts.selected_fields.is_empty() => Err(error(
            WorthUiProjectionDeclarationErrorKind::CollectionFieldCount,
            "collection projection requires at least one selected field",
        )),
        WorthUiProjectionShape::Collection
            if parts.row_identity_field.is_none() || parts.collection_policy.is_none() =>
        {
            Err(error(
                WorthUiProjectionDeclarationErrorKind::MissingClause,
                "collection projection requires row identity and collection policy",
            ))
        }
        WorthUiProjectionShape::Scalar | WorthUiProjectionShape::Collection => Ok(()),
    }
}

fn validate_selected_fields(
    selected_fields: &[String],
) -> Result<(), WorthUiProjectionDeclarationError> {
    let mut distinct_fields = BTreeSet::new();
    for field in selected_fields {
        validate_identity("selected field", field)?;
        if !distinct_fields.insert(field.as_str()) {
            return Err(error(
                WorthUiProjectionDeclarationErrorKind::DuplicateSelectedField,
                format!("selected field `{field}` is declared more than once"),
            ));
        }
    }
    Ok(())
}

impl WorthUiProjectionCollectionSelection {
    pub fn new(
        selected_fields: impl IntoIterator<Item = impl Into<String>>,
        lifecycle: WorthUiProjectionLifecycle,
        policy: WorthUiProjectionCollectionPolicy,
    ) -> Self {
        Self {
            selected_fields: selected_fields.into_iter().map(Into::into).collect(),
            lifecycle,
            policy,
        }
    }
}

impl WorthUiProjectionShape {
    pub(crate) const fn canonical_token(self) -> &'static str {
        match self {
            Self::Scalar => "scalar",
            Self::Collection => "collection",
        }
    }
}

impl WorthUiProjectionNativeFamily {
    pub(crate) const fn canonical_token(self) -> &'static str {
        match self {
            Self::Text => "text",
            Self::Boolean => "boolean",
        }
    }
}

impl WorthUiProjectionLifecycle {
    pub(crate) const fn canonical_token(self) -> &'static str {
        match self {
            Self::Snapshot => "snapshot",
            Self::Live => "live",
        }
    }
}

impl WorthUiProjectionCollectionPolicy {
    pub const fn new(requires_complete_result: bool, permits_continuation: bool) -> Self {
        Self {
            requires_complete_result,
            permits_continuation,
        }
    }

    pub const fn requires_complete_result(self) -> bool {
        self.requires_complete_result
    }

    pub const fn permits_continuation(self) -> bool {
        self.permits_continuation
    }
}

impl WorthUiProjectionRequirementIdentity {
    pub const fn as_u64(self) -> u64 {
        self.0
    }
}

impl WorthUiProjectionDeclarationError {
    pub(crate) fn new(
        kind: WorthUiProjectionDeclarationErrorKind,
        detail: impl Into<Arc<str>>,
    ) -> Self {
        Self {
            kind,
            detail: detail.into(),
        }
    }

    pub fn kind(&self) -> WorthUiProjectionDeclarationErrorKind {
        self.kind
    }

    pub fn detail(&self) -> &str {
        self.detail.as_ref()
    }
}

fn validate_identity(label: &str, value: &str) -> Result<(), WorthUiProjectionDeclarationError> {
    if value.is_empty() || value.trim() != value {
        return Err(error(
            WorthUiProjectionDeclarationErrorKind::EmptyIdentity,
            format!("{label} identity is empty or has surrounding whitespace"),
        ));
    }
    Ok(())
}

fn error(
    kind: WorthUiProjectionDeclarationErrorKind,
    detail: impl Into<Arc<str>>,
) -> WorthUiProjectionDeclarationError {
    WorthUiProjectionDeclarationError::new(kind, detail)
}

fn identity_for(parts: &WorthUiProjectionRequirementParts) -> WorthUiProjectionRequirementIdentity {
    let mut digest = fold(0xcbf2_9ce4_8422_2325, &parts.declaration_identity);
    digest = fold(digest, &parts.view_identity);
    digest = fold(digest, parts.shape.canonical_token());
    for field in &parts.selected_fields {
        digest = fold(digest, field);
    }
    digest = fold(digest, parts.row_identity_field.as_deref().unwrap_or("-"));
    digest = fold(digest, parts.native_family.canonical_token());
    digest = fold(digest, parts.lifecycle.canonical_token());
    if let Some(policy) = parts.collection_policy {
        digest = fold(
            digest,
            if policy.requires_complete_result {
                "complete"
            } else {
                "partial"
            },
        );
        digest = fold(
            digest,
            if policy.permits_continuation {
                "continuation"
            } else {
                "bounded"
            },
        );
    }
    WorthUiProjectionRequirementIdentity(digest)
}

fn fold(mut digest: u64, text: &str) -> u64 {
    digest ^= text.len() as u64;
    digest = digest.wrapping_mul(0x100_0000_01b3);
    for byte in text.as_bytes() {
        digest ^= u64::from(*byte);
        digest = digest.wrapping_mul(0x100_0000_01b3);
    }
    digest
}
