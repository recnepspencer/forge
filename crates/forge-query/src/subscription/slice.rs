use crate::identity::hash_parts;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum QuerySubscriptionSliceKind {
    AuthorizedProjection,
    Membership,
    Ordering,
    Grouping,
    RelationScope,
    ViewShapeMetadata,
}

impl QuerySubscriptionSliceKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::AuthorizedProjection => "authorized_projection",
            Self::Membership => "membership",
            Self::Ordering => "ordering",
            Self::Grouping => "grouping",
            Self::RelationScope => "relation_scope",
            Self::ViewShapeMetadata => "view_shape_metadata",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct QuerySubscriptionSlicePart {
    kind: QuerySubscriptionSliceKind,
    ordinal: usize,
}

impl QuerySubscriptionSlicePart {
    pub(super) fn new(kind: QuerySubscriptionSliceKind, ordinal: usize) -> Self {
        Self { kind, ordinal }
    }

    pub fn kind(&self) -> &QuerySubscriptionSliceKind {
        &self.kind
    }

    pub fn ordinal(&self) -> usize {
        self.ordinal
    }

    pub(super) fn canonical_part(&self) -> String {
        format!("{}:{}", self.kind.as_str(), self.ordinal)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QuerySubscriptionSliceIntent {
    parts: Vec<QuerySubscriptionSlicePart>,
    digest: String,
}

impl QuerySubscriptionSliceIntent {
    pub(super) fn from_canonical_parts(mut parts: Vec<QuerySubscriptionSlicePart>) -> Self {
        parts.sort();
        parts.dedup();
        let digest_parts = parts
            .iter()
            .map(QuerySubscriptionSlicePart::canonical_part)
            .collect::<Vec<_>>();
        Self {
            parts,
            digest: hash_parts(&digest_parts),
        }
    }

    pub fn parts(&self) -> &[QuerySubscriptionSlicePart] {
        &self.parts
    }

    pub fn digest(&self) -> &str {
        &self.digest
    }

    pub fn len(&self) -> usize {
        self.parts.len()
    }
}
