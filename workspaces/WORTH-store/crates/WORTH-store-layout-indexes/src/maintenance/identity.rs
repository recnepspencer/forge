use crate::strategy::S8LayoutStrategyFamily;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8DerivedIndexIdentityParity {
    Exact,
    SourceArtifactDoesNotProveIdentity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8DerivedIndexOrderingParity {
    Exact,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8DerivedIndexCoverageParity {
    Exact,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8DerivedIndexCostEnvelopeParity {
    DeclaredEnvelopeMatched,
    SourceArtifactDoesNotProveDeclaredEnvelope,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8DerivedIndexCounterShapeParity {
    ExactDeterministicPhysicalShape,
    StrategyDoesNotClaimDeterministicPhysicalShape,
}

pub(crate) const fn declared_counter_shape_parity(
    family: S8LayoutStrategyFamily,
) -> S8DerivedIndexCounterShapeParity {
    match family {
        S8LayoutStrategyFamily::BaselineBTreeRange => {
            S8DerivedIndexCounterShapeParity::ExactDeterministicPhysicalShape
        }
        _ => S8DerivedIndexCounterShapeParity::StrategyDoesNotClaimDeterministicPhysicalShape,
    }
}
