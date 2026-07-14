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
        expected: forge_store_security::StoreSecurityScopeIdentity,
        actual: forge_store_security::StoreSecurityScopeIdentity,
    },
    SourceStoreAuthorityMismatch {
        expected: forge_store_authority::StoreCurrentAuthorityIdentity,
        actual: forge_store_authority::StoreCurrentAuthorityIdentity,
    },
    ParityRowsMustBeCanonical,
    ParityKeysMustBeUnique,
    ParityCounterShapeMustBeCanonical,
}
