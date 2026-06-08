use super::input::{
    ForgeQueryDeclarationCanonicalEntry, ForgeQueryDeclarationCanonicalEntryKind,
    ForgeQueryDeclarationCanonicalValue,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ForgeQueryAsyncDeclarationSupport {
    Unsupported,
    CanonicalIdentityOnly,
    DeferredDebt,
}

impl ForgeQueryAsyncDeclarationSupport {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Unsupported => "unsupported",
            Self::CanonicalIdentityOnly => "canonical_identity_only",
            Self::DeferredDebt => "deferred_debt",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ForgeQueryAsyncSourceFamily {
    BridgeResource,
    ExternalResource,
    HostResource,
}

impl ForgeQueryAsyncSourceFamily {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::BridgeResource => "bridge-resource",
            Self::ExternalResource => "external-resource",
            Self::HostResource => "host-resource",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ForgeQueryAsyncLoadingPosture {
    Blocking,
    BackgroundRefresh,
}

impl ForgeQueryAsyncLoadingPosture {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Blocking => "blocking",
            Self::BackgroundRefresh => "background-refresh",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ForgeQueryAsyncFailurePosture {
    FailClosed,
    RetainStaleValue,
}

impl ForgeQueryAsyncFailurePosture {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::FailClosed => "fail-closed",
            Self::RetainStaleValue => "retain-stale-value",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ForgeQueryAsyncRequestIdentityPart {
    key: String,
    value: String,
}

impl ForgeQueryAsyncRequestIdentityPart {
    pub fn text(key: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            value: value.into(),
        }
    }

    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ForgeQueryAsyncDeclarationClause {
    ResourceRequest {
        source_family: ForgeQueryAsyncSourceFamily,
        loading_posture: ForgeQueryAsyncLoadingPosture,
        failure_posture: ForgeQueryAsyncFailurePosture,
        request_identity: Vec<ForgeQueryAsyncRequestIdentityPart>,
    },
    CompletionRequest {
        source_family: ForgeQueryAsyncSourceFamily,
        failure_posture: ForgeQueryAsyncFailurePosture,
        request_identity: Vec<ForgeQueryAsyncRequestIdentityPart>,
    },
}

impl ForgeQueryAsyncDeclarationClause {
    pub fn resource_request(
        source_family: ForgeQueryAsyncSourceFamily,
        loading_posture: ForgeQueryAsyncLoadingPosture,
        failure_posture: ForgeQueryAsyncFailurePosture,
        request_identity: Vec<ForgeQueryAsyncRequestIdentityPart>,
    ) -> Self {
        Self::ResourceRequest {
            source_family,
            loading_posture,
            failure_posture,
            request_identity,
        }
    }

    pub fn completion_request(
        source_family: ForgeQueryAsyncSourceFamily,
        failure_posture: ForgeQueryAsyncFailurePosture,
        request_identity: Vec<ForgeQueryAsyncRequestIdentityPart>,
    ) -> Self {
        Self::CompletionRequest {
            source_family,
            failure_posture,
            request_identity,
        }
    }

    fn family_key(&self) -> &'static str {
        match self {
            Self::ResourceRequest { .. } => "resource-request",
            Self::CompletionRequest { .. } => "completion-request",
        }
    }

    fn normalized_key(&self) -> String {
        match self {
            Self::ResourceRequest {
                source_family,
                loading_posture,
                failure_posture,
                request_identity,
            } => format!(
                "resource-request:{}:{}:{}:{}",
                source_family.as_str(),
                loading_posture.as_str(),
                failure_posture.as_str(),
                normalized_request_identity_key(request_identity)
            ),
            Self::CompletionRequest {
                source_family,
                failure_posture,
                request_identity,
            } => format!(
                "completion-request:{}:{}:{}",
                source_family.as_str(),
                failure_posture.as_str(),
                normalized_request_identity_key(request_identity)
            ),
        }
    }
}

pub(crate) fn normalize_async_resource_clauses(
    clauses: Vec<ForgeQueryAsyncDeclarationClause>,
) -> Vec<ForgeQueryAsyncDeclarationClause> {
    let mut clauses = clauses
        .into_iter()
        .map(normalize_async_resource_clause)
        .collect::<Vec<_>>();
    clauses.sort_by_cached_key(ForgeQueryAsyncDeclarationClause::normalized_key);
    clauses.dedup();
    clauses
}

pub(crate) fn async_resource_entries(
    clauses: &[ForgeQueryAsyncDeclarationClause],
) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
    clauses
        .iter()
        .enumerate()
        .flat_map(|(index, clause)| clause_entries(index, clause))
        .collect()
}

fn normalize_async_resource_clause(
    clause: ForgeQueryAsyncDeclarationClause,
) -> ForgeQueryAsyncDeclarationClause {
    match clause {
        ForgeQueryAsyncDeclarationClause::ResourceRequest {
            source_family,
            loading_posture,
            failure_posture,
            request_identity,
        } => ForgeQueryAsyncDeclarationClause::ResourceRequest {
            source_family,
            loading_posture,
            failure_posture,
            request_identity: normalize_request_identity(request_identity),
        },
        ForgeQueryAsyncDeclarationClause::CompletionRequest {
            source_family,
            failure_posture,
            request_identity,
        } => ForgeQueryAsyncDeclarationClause::CompletionRequest {
            source_family,
            failure_posture,
            request_identity: normalize_request_identity(request_identity),
        },
    }
}

fn normalize_request_identity(
    request_identity: Vec<ForgeQueryAsyncRequestIdentityPart>,
) -> Vec<ForgeQueryAsyncRequestIdentityPart> {
    let mut request_identity = request_identity;
    request_identity.sort();
    request_identity.dedup();
    request_identity
}

fn normalized_request_identity_key(
    request_identity: &[ForgeQueryAsyncRequestIdentityPart],
) -> String {
    request_identity
        .iter()
        .map(|part| format!("{}={}", part.key(), part.value()))
        .collect::<Vec<_>>()
        .join("|")
}

fn clause_entries(
    index: usize,
    clause: &ForgeQueryAsyncDeclarationClause,
) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
    let base = format!("declaration.async_resource.{index}");
    let mut entries = vec![text_shape_entry(
        format!("{base}.family"),
        clause.family_key(),
    )];

    match clause {
        ForgeQueryAsyncDeclarationClause::ResourceRequest {
            source_family,
            loading_posture,
            failure_posture,
            request_identity,
        } => {
            entries.push(text_shape_entry(
                format!("{base}.source_family"),
                source_family.as_str(),
            ));
            entries.push(text_shape_entry(
                format!("{base}.loading_posture"),
                loading_posture.as_str(),
            ));
            entries.push(text_shape_entry(
                format!("{base}.failure_posture"),
                failure_posture.as_str(),
            ));
            entries.extend(request_identity_entries(
                format!("{base}.request_identity"),
                request_identity,
            ));
        }
        ForgeQueryAsyncDeclarationClause::CompletionRequest {
            source_family,
            failure_posture,
            request_identity,
        } => {
            entries.push(text_shape_entry(
                format!("{base}.source_family"),
                source_family.as_str(),
            ));
            entries.push(text_shape_entry(
                format!("{base}.failure_posture"),
                failure_posture.as_str(),
            ));
            entries.extend(request_identity_entries(
                format!("{base}.request_identity"),
                request_identity,
            ));
        }
    }

    entries
}

fn request_identity_entries(
    base: String,
    request_identity: &[ForgeQueryAsyncRequestIdentityPart],
) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
    request_identity
        .iter()
        .enumerate()
        .flat_map(|(index, part)| {
            [
                ForgeQueryDeclarationCanonicalEntry::new(
                    format!("{base}.{index}.key"),
                    ForgeQueryDeclarationCanonicalEntryKind::Identity,
                    ForgeQueryDeclarationCanonicalValue::ExactText(part.key().to_string()),
                ),
                ForgeQueryDeclarationCanonicalEntry::new(
                    format!("{base}.{index}.value"),
                    ForgeQueryDeclarationCanonicalEntryKind::Identity,
                    ForgeQueryDeclarationCanonicalValue::ExactText(part.value().to_string()),
                ),
            ]
        })
        .collect()
}

fn text_shape_entry(
    locus: impl Into<String>,
    value: impl Into<String>,
) -> ForgeQueryDeclarationCanonicalEntry {
    ForgeQueryDeclarationCanonicalEntry::new(
        locus,
        ForgeQueryDeclarationCanonicalEntryKind::Shape,
        ForgeQueryDeclarationCanonicalValue::ExactText(value.into()),
    )
}
