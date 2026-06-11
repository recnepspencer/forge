use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

use crate::evidence_identity::{
    ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag,
};

macro_rules! causal_identity_type {
    ($name:ident) => {
        #[derive(Clone, Debug, Eq, PartialEq)]
        pub struct $name(ForgeQueryEvidenceIdentity);

        impl $name {
            pub(crate) fn from_identity(identity: ForgeQueryEvidenceIdentity) -> Self {
                Self(identity)
            }

            pub fn as_str(&self) -> &str {
                self.0.as_str()
            }
        }

        impl From<ForgeQueryEvidenceIdentity> for $name {
            fn from(value: ForgeQueryEvidenceIdentity) -> Self {
                Self::from_identity(value)
            }
        }

        impl AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                self.as_str()
            }
        }

        impl Ord for $name {
            fn cmp(&self, other: &Self) -> Ordering {
                self.as_str().cmp(other.as_str())
            }
        }

        impl PartialOrd for $name {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }

        impl Hash for $name {
            fn hash<H: Hasher>(&self, state: &mut H) {
                self.as_str().hash(state);
            }
        }
    };
}

macro_rules! causal_handle_string_type {
    ($name:ident) => {
        #[derive(Clone, Debug, Eq, PartialEq)]
        pub struct $name(String);

        impl $name {
            pub(crate) fn from_handle(handle: impl Into<String>) -> Self {
                Self(handle.into())
            }

            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl From<String> for $name {
            fn from(value: String) -> Self {
                Self::from_handle(value)
            }
        }

        impl From<&str> for $name {
            fn from(value: &str) -> Self {
                Self::from_handle(value)
            }
        }

        impl AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                self.as_str()
            }
        }

        impl Ord for $name {
            fn cmp(&self, other: &Self) -> Ordering {
                self.as_str().cmp(other.as_str())
            }
        }

        impl PartialOrd for $name {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }

        impl Hash for $name {
            fn hash<H: Hasher>(&self, state: &mut H) {
                self.as_str().hash(state);
            }
        }
    };
}

causal_identity_type!(CausalObservationReceiptIdentity);
causal_identity_type!(CausalObservationQueryIdentity);
causal_identity_type!(CausalObservationBasisIdentity);
causal_identity_type!(CausalObservationTargetIdentity);
causal_identity_type!(CausalResultShapeContextIdentity);
causal_identity_type!(CausalQueryObservationReceiptIdentity);
causal_identity_type!(CausalObservationAnchorDigest);
causal_identity_type!(CausalObservationAnchorCountersIdentity);
causal_identity_type!(CausalObservationAnchorFailureIdentity);
causal_identity_type!(CausalEvidenceReferenceReceiptIdentity);
causal_identity_type!(CausalEvidenceReferenceResolutionCountersIdentity);
causal_identity_type!(CausalEvidenceReferenceResolutionDenialIdentity);
causal_identity_type!(CausalEvidenceReferenceIndexIdentity);
causal_identity_type!(CausalEvidenceReferenceIndexRecordIdentity);
causal_identity_type!(CausalEvidenceReferenceIndexErrorIdentity);
causal_handle_string_type!(CausalEvidenceReferenceDigest);

impl From<ForgeQueryEvidenceIdentity> for CausalEvidenceReferenceDigest {
    fn from(value: ForgeQueryEvidenceIdentity) -> Self {
        Self::from_handle(value.as_str())
    }
}

pub(in crate::runtime) enum CausalEvidenceReferenceInput {
    Typed(CausalEvidenceReferenceDigest),
    Source(String),
}

impl From<CausalEvidenceReferenceDigest> for CausalEvidenceReferenceInput {
    fn from(value: CausalEvidenceReferenceDigest) -> Self {
        Self::Typed(value)
    }
}

impl From<&str> for CausalEvidenceReferenceInput {
    fn from(value: &str) -> Self {
        Self::Source(value.to_string())
    }
}

impl From<String> for CausalEvidenceReferenceInput {
    fn from(value: String) -> Self {
        Self::Source(value)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CausalObservationTargetHandle {
    rendered: String,
    identity: CausalObservationTargetIdentity,
}

impl CausalObservationTargetHandle {
    pub(crate) fn from_rendered(rendered: impl Into<String>) -> Self {
        let rendered = rendered.into();
        let identity =
            ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::CausalObservationTarget)
                .field_value(ForgeQueryEvidenceTag::new("rendered"), &rendered)
                .seal()
                .into();
        Self { rendered, identity }
    }

    pub fn rendered(&self) -> &str {
        &self.rendered
    }

    pub fn identity(&self) -> &CausalObservationTargetIdentity {
        &self.identity
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CausalResultShapeContextHandle {
    rendered: String,
    identity: CausalResultShapeContextIdentity,
}

impl CausalResultShapeContextHandle {
    pub(crate) fn from_rendered(rendered: impl Into<String>) -> Self {
        let rendered = rendered.into();
        let identity =
            ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::CausalResultShapeContext)
                .field_value(ForgeQueryEvidenceTag::new("rendered"), &rendered)
                .seal()
                .into();
        Self { rendered, identity }
    }

    pub fn rendered(&self) -> &str {
        &self.rendered
    }

    pub fn identity(&self) -> &CausalResultShapeContextIdentity {
        &self.identity
    }
}
