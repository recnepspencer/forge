use super::input::{
    WorthQueryDeclarationCanonicalEntry, WorthQueryDeclarationCanonicalEntryKind,
    WorthQueryDeclarationCanonicalValue,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum WorthQueryAsyncDeclarationSupport {
    Unsupported,
    CanonicalIdentityOnly,
    DeferredDebt,
}

impl WorthQueryAsyncDeclarationSupport {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Unsupported => "unsupported",
            Self::CanonicalIdentityOnly => "canonical_identity_only",
            Self::DeferredDebt => "deferred_debt",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum WorthQueryAsyncSourceFamily {
    BridgeResource,
    ExternalResource,
    HostResource,
}

impl WorthQueryAsyncSourceFamily {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::BridgeResource => "bridge-resource",
            Self::ExternalResource => "external-resource",
            Self::HostResource => "host-resource",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum WorthQueryAsyncLoadingPosture {
    Blocking,
    BackgroundRefresh,
}

impl WorthQueryAsyncLoadingPosture {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Blocking => "blocking",
            Self::BackgroundRefresh => "background-refresh",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum WorthQueryAsyncFailurePosture {
    FailClosed,
    RetainStaleValue,
}

impl WorthQueryAsyncFailurePosture {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::FailClosed => "fail-closed",
            Self::RetainStaleValue => "retain-stale-value",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct WorthQueryAsyncRequestIdentityPart {
    key: String,
    value: String,
}

impl WorthQueryAsyncRequestIdentityPart {
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
pub enum WorthQueryAsyncDeclarationClause {
    ResourceRequest {
        source_family: WorthQueryAsyncSourceFamily,
        loading_posture: WorthQueryAsyncLoadingPosture,
        failure_posture: WorthQueryAsyncFailurePosture,
        request_identity: Vec<WorthQueryAsyncRequestIdentityPart>,
    },
    CompletionRequest {
        source_family: WorthQueryAsyncSourceFamily,
        failure_posture: WorthQueryAsyncFailurePosture,
        request_identity: Vec<WorthQueryAsyncRequestIdentityPart>,
    },
}

impl WorthQueryAsyncDeclarationClause {
    pub fn resource_request(
        source_family: WorthQueryAsyncSourceFamily,
        loading_posture: WorthQueryAsyncLoadingPosture,
        failure_posture: WorthQueryAsyncFailurePosture,
        request_identity: Vec<WorthQueryAsyncRequestIdentityPart>,
    ) -> Self {
        Self::ResourceRequest {
            source_family,
            loading_posture,
            failure_posture,
            request_identity,
        }
    }

    pub fn completion_request(
        source_family: WorthQueryAsyncSourceFamily,
        failure_posture: WorthQueryAsyncFailurePosture,
        request_identity: Vec<WorthQueryAsyncRequestIdentityPart>,
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
    clauses: Vec<WorthQueryAsyncDeclarationClause>,
) -> Vec<WorthQueryAsyncDeclarationClause> {
    let mut clauses = clauses
        .into_iter()
        .map(normalize_async_resource_clause)
        .collect::<Vec<_>>();
    clauses.sort_by_cached_key(WorthQueryAsyncDeclarationClause::normalized_key);
    clauses.dedup();
    clauses
}

pub(crate) fn async_resource_entries(
    clauses: &[WorthQueryAsyncDeclarationClause],
) -> Vec<WorthQueryDeclarationCanonicalEntry> {
    clauses
        .iter()
        .enumerate()
        .flat_map(|(index, clause)| clause_entries(index, clause))
        .collect()
}

fn normalize_async_resource_clause(
    clause: WorthQueryAsyncDeclarationClause,
) -> WorthQueryAsyncDeclarationClause {
    match clause {
        WorthQueryAsyncDeclarationClause::ResourceRequest {
            source_family,
            loading_posture,
            failure_posture,
            request_identity,
        } => WorthQueryAsyncDeclarationClause::ResourceRequest {
            source_family,
            loading_posture,
            failure_posture,
            request_identity: normalize_request_identity(request_identity),
        },
        WorthQueryAsyncDeclarationClause::CompletionRequest {
            source_family,
            failure_posture,
            request_identity,
        } => WorthQueryAsyncDeclarationClause::CompletionRequest {
            source_family,
            failure_posture,
            request_identity: normalize_request_identity(request_identity),
        },
    }
}

fn normalize_request_identity(
    request_identity: Vec<WorthQueryAsyncRequestIdentityPart>,
) -> Vec<WorthQueryAsyncRequestIdentityPart> {
    let mut request_identity = request_identity;
    request_identity.sort();
    request_identity.dedup();
    request_identity
}

fn normalized_request_identity_key(
    request_identity: &[WorthQueryAsyncRequestIdentityPart],
) -> String {
    request_identity
        .iter()
        .map(|part| format!("{}={}", part.key(), part.value()))
        .collect::<Vec<_>>()
        .join("|")
}

fn clause_entries(
    index: usize,
    clause: &WorthQueryAsyncDeclarationClause,
) -> Vec<WorthQueryDeclarationCanonicalEntry> {
    let base = format!("declaration.async_resource.{index}");
    let mut entries = vec![text_shape_entry(
        format!("{base}.family"),
        clause.family_key(),
    )];

    match clause {
        WorthQueryAsyncDeclarationClause::ResourceRequest {
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
        WorthQueryAsyncDeclarationClause::CompletionRequest {
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
    request_identity: &[WorthQueryAsyncRequestIdentityPart],
) -> Vec<WorthQueryDeclarationCanonicalEntry> {
    request_identity
        .iter()
        .enumerate()
        .flat_map(|(index, part)| {
            [
                WorthQueryDeclarationCanonicalEntry::new(
                    format!("{base}.{index}.key"),
                    WorthQueryDeclarationCanonicalEntryKind::Identity,
                    WorthQueryDeclarationCanonicalValue::ExactText(part.key().to_string()),
                ),
                WorthQueryDeclarationCanonicalEntry::new(
                    format!("{base}.{index}.value"),
                    WorthQueryDeclarationCanonicalEntryKind::Identity,
                    WorthQueryDeclarationCanonicalValue::ExactText(part.value().to_string()),
                ),
            ]
        })
        .collect()
}

fn text_shape_entry(
    locus: impl Into<String>,
    value: impl Into<String>,
) -> WorthQueryDeclarationCanonicalEntry {
    WorthQueryDeclarationCanonicalEntry::new(
        locus,
        WorthQueryDeclarationCanonicalEntryKind::Shape,
        WorthQueryDeclarationCanonicalValue::ExactText(value.into()),
    )
}
