use crate::strategy::{LayoutStrategyFamily, StrategyDenial, StrategyRebuildSourceRequirement};
use crate::LayoutMaterializationSourceKind;

use super::source::DerivedIndexRebuildSourceInput;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DerivedIndexRebuildDenied {
    StrategyDenied {
        denial: StrategyDenial,
    },
    SourceInputIsNotAuthority {
        source: Box<DerivedIndexRebuildSourceInput>,
    },
    RebuildShapeRequired {
        family: LayoutStrategyFamily,
    },
    SourceArtifactDoesNotMatchStrategy {
        required: StrategyRebuildSourceRequirement,
        source: &'static str,
    },
    SourceMaterializationIdentityMismatch {
        materialization: LayoutMaterializationSourceKind,
        source: LayoutMaterializationSourceKind,
    },
    SourceSecurityScopeMismatch {
        expected: worth_store_security::StoreSecurityScopeIdentity,
        actual: worth_store_security::StoreSecurityScopeIdentity,
    },
    SourceStoreAuthorityMismatch {
        expected: worth_store_authority::StoreCurrentAuthorityIdentity,
        actual: worth_store_authority::StoreCurrentAuthorityIdentity,
    },
    ParityRowsMustBeCanonical,
    ParityKeysMustBeUnique,
    ParityCounterShapeMustBeCanonical,
}
